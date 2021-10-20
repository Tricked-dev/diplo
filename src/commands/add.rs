use crate::{
    load_config::{update_config_json, update_config_toml},
    update_deno::{get_latest_std, Versions, HTTP_CLIENT},
    CONFIG, DIPLO_CONFIG,
};
use anyhow::Result;
use clap::ArgMatches;
use colored::Colorize;
use hyper::body::Buf;
use serde_json::json;
use std::fs::read_to_string;
use toml_edit::{value, Document};

pub async fn exec(sub_m: &ArgMatches) -> Result<()> {
    if let Some(modules) = sub_m.values_of("module") {
        for module in modules {
            if sub_m.is_present("std") {
                let latest_std = get_latest_std().await;

                let std_module = &format!("https://deno.land/std@{}/{}/mod.ts", latest_std, module);
                let mut deps = CONFIG.dependencies.as_ref().unwrap().clone();
                deps.insert((&module).to_string(), std_module.to_string());
                //Errors otherwise
                if DIPLO_CONFIG.ends_with(".toml") {
                    let data = read_to_string(&*DIPLO_CONFIG);
                    if let Ok(data) = data {
                        let mut document = data.parse::<Document>().unwrap();
                        for (name, val) in deps.iter() {
                            document["dependencies"][name] = value(val);
                        }
                        update_config_toml(document);
                        println!(
                            "Successfully added {} to the dependencies",
                            std_module.green()
                        );
                    } else {
                        println!("Could not locate {}", &*DIPLO_CONFIG);
                        println!("please initialize diplo with diplo init");
                    }
                } else if let true = update_config_json(json!({ "dependencies": deps })) {
                    println!(
                        "Successfully added {} to the dependencies",
                        std_module.green()
                    );
                }
            } else {
                let res = HTTP_CLIENT
                    .get(
                        format!("https://cdn.deno.land/{}/meta/versions.json", &module)
                            .parse()
                            .unwrap(),
                    )
                    .await
                    .unwrap();

                let body = hyper::body::aggregate(res).await.unwrap();

                let json: Result<Versions, serde_json::Error> =
                    serde_json::from_reader(body.reader());
                if let Ok(json) = json {
                    let mut deps = CONFIG.dependencies.as_ref().unwrap().clone();
                    deps.insert(
                        (&module).to_string(),
                        format!("https://deno.land/x/{}@{}/mod.ts", module, json.latest),
                    );
                    //Errors otherwise
                    if DIPLO_CONFIG.ends_with(".toml") {
                        //Cant error cause it would default to json
                        let data = read_to_string(&*DIPLO_CONFIG);
                        if let Ok(data) = data {
                            let mut document = data.parse::<Document>().unwrap();
                            for (name, val) in deps.iter() {
                                document["dependencies"][name] = value(val);
                            }
                            update_config_toml(document);
                            println!(
                                "Successfully added {}@{} to the dependencies",
                                module.yellow(),
                                json.latest.yellow()
                            )
                        } else {
                            println!("Could not locate {}", &*DIPLO_CONFIG);
                            println!("please initialize diplo with diplo init");
                        }
                    } else if let true = update_config_json(json!({ "dependencies": deps })) {
                        println!(
                            "Successfully added {}@{} to the dependencies",
                            module.yellow(),
                            json.latest.yellow()
                        )
                    }
                } else {
                    println!("No module named {} found", module)
                }
            }
        }
    }
    Ok(())
}
