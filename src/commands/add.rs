use anyhow::Result;
use clap::ArgMatches;
use diplo::{
    info,
    load_config::update_config,
    term::print_inner,
    update_deno::{get_latest_std, Versions, HTTP_CLIENT},
    CONFIG,
};
use serde_json::json;

pub async fn exec(sub_m: Box<&ArgMatches>) -> Result<()> {
    if let Some(module) = sub_m.value_of("module") {
        if sub_m.is_present("std") {
            let latest_std = get_latest_std().await;

            let data = &format!("https://deno.land/std@{}/{}/mod.ts", latest_std, module);
            let mut deps = CONFIG.dependencies.as_ref().unwrap().clone();
            deps.insert((&module).to_string(), data.to_string());
            //Errors otherwise
            if let true = update_config(json!({ "dependencies": deps })) {
                info!("Succesfully added {} to the dependencies", data)
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
                if let true = update_config(json!({ "dependencies": deps })) {
                    info!(
                        "Succesfully added {}@{} to the dependencies",
                        module, json.latest
                    )
                }
            } else {
                info!("No module named {} found", module)
            }
        }
    }
    Ok(())
}
