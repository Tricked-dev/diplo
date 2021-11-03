use crate::{
    load_config::{update_config_toml},
    update_deno::{get_latest_std, Versions, HTTP_CLIENT},
    utils::run_utils::ensure_dependencies, DIPLO_CONFIG,
};
use anyhow::Result;
use clap::ArgMatches;
use colored::Colorize;
use hyper::body::Buf;

use std::fs::read_to_string;
use toml_edit::{value, Document};

pub async fn exec(sub_m: &ArgMatches) -> Result<()> {
    if let Some(modules) = sub_m.values_of("module") {
        for module in modules {
            if sub_m.is_present("std") {
                let latest_std = get_latest_std().await?;

                let std_module = &format!("https://deno.land/std@{}/{}/mod.ts", latest_std, module);

                let data = read_to_string(&*DIPLO_CONFIG);
                if let Ok(data) = data {
                    let mut document = data.parse::<Document>()?;

                    document["dependencies"][module] = value(std_module.to_string());

                    update_config_toml(document);
                    println!(
                        "Successfully added {} to the dependencies",
                        std_module.green()
                    );
                } else {
                    println!("Could not locate {}", &*DIPLO_CONFIG);
                    println!("please initialize diplo with diplo init");
                }
            } else {
                let res = HTTP_CLIENT
                    .get(format!("https://cdn.deno.land/{}/meta/versions.json", &module).parse()?)
                    .await
                    .unwrap();

                let body = hyper::body::aggregate(res).await?;

                let json: Result<Versions, serde_json::Error> =
                    serde_json::from_reader(body.reader());
                if let Ok(json) = json {
                    let data = read_to_string(&*DIPLO_CONFIG);
                    if let Ok(data) = data {
                        let mut document = data.parse::<Document>()?;

                        document["dependencies"][(&module).to_string()] = value(format!(
                            "https://deno.land/x/{}@{}/mod.ts",
                            module, json.latest
                        ));

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
                } else {
                    println!("No module named {} found", module)
                }
            }
        }
    }
    ensure_dependencies()?;
    Ok(())
}
