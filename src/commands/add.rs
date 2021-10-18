use anyhow::Result;
use clap::ArgMatches;
use diplo::{
    error, info,
    load_config::{create_deps, update_config},
    term::print_inner,
    update_deno::{get_latest_std, update_deps, Versions, HTTP_CLIENT},
    warn,
    watcher::{get_config, DiploHandler},
    CONFIG, DIPLOJSON, DOTDIPLO,
};
use serde_json::json;
use std::{
    fs::{self, write},
    process::Command,
};
use watchexec::{run::ExecHandler, watch};

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
