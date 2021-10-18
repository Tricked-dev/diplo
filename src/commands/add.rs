use anyhow::Result;
use clap::ArgMatches;
use diplo::{
    info,
    load_config::{update_config_json, update_config_toml},
    term::print_inner,
    update_deno::{get_latest_std, Versions, HTTP_CLIENT},
    CONFIG, DIPLO_CONFIG,
};
use serde_json::json;
use std::fs::read_to_string;
use toml_edit::{value, Document};

pub async fn exec(sub_m: &ArgMatches) -> Result<()> {
    if let Some(modules) = sub_m.values_of("module") {
        for module in modules {
            if sub_m.is_present("std") {
                let latest_std = get_latest_std().await;

                let data = &format!("https://deno.land/std@{}/{}/mod.ts", latest_std, module);
                let mut deps = CONFIG.dependencies.as_ref().unwrap().clone();
                deps.insert((&module).to_string(), data.to_string());
                //Errors otherwise
                if DIPLO_CONFIG.ends_with(".toml") {
                    //Cant error cause it would default to json
                    let data = read_to_string(&*DIPLO_CONFIG).unwrap();
                    let mut document = data.parse::<Document>().unwrap();
                    for (name, val) in deps.iter() {
                        document["dependencies"][name] = value(val);
                    }
                    update_config_toml(document);
                    info!("Successfully added {} to the dependencies", data);
                } else if let true = update_config_json(json!({ "dependencies": deps })) {
                    info!("Successfully added {} to the dependencies", data)
                }
            } else {
                let res = HTTP_CLIENT
                    .get(format!(
                        "https://cdn.deno.land/{}/meta/versions.json",
                        &module
                    ))
                    .header("user-agent", "diplo")
                    .send()
                    .await
                    .unwrap();
                let text = res.text().await.unwrap();

                let json: Result<Versions, serde_json::Error> = serde_json::from_str(&text);
                if let Ok(json) = json {
                    let mut deps = CONFIG.dependencies.as_ref().unwrap().clone();
                    deps.insert(
                        (&module).to_string(),
                        format!("https://deno.land/x/{}@{}/mod.ts", module, json.latest),
                    );
                    //Errors otherwise
                    if DIPLO_CONFIG.ends_with(".toml") {
                        //Cant error cause it would default to json
                        let data = read_to_string(&*DIPLO_CONFIG).unwrap();
                        let mut document = data.parse::<Document>().unwrap();
                        for (name, val) in deps.iter() {
                            document["dependencies"][name] = value(val);
                        }
                        update_config_toml(document);
                        info!(
                            "Successfully added {}@{} to the dependencies",
                            module, json.latest
                        )
                    } else if let true = update_config_json(json!({ "dependencies": deps })) {
                        info!(
                            "Successfully added {}@{} to the dependencies",
                            module, json.latest
                        )
                    }
                } else {
                    info!("No module named {} found", module)
                }
            }
        }
    }
    Ok(())
}
