use crate::{
    load_config::create_deps,
    watcher::{get_config, DiploHandler},
    CONFIG, DIPLO_CONFIG, DOTDIPLO,
};
use anyhow::Result;
use clap::ArgMatches;
use colored::Colorize;
use serde_json::json;
use std::{fs::write, process::Command};
use watchexec::{run::ExecHandler, watch};

pub fn exec(sub_m: &ArgMatches) -> Result<()> {
    if let Some(script) = sub_m.value_of("script") {
        let mut extra_args: Vec<String> = vec![];

        if let Some(dependencies) = &CONFIG.dependencies {
            create_deps(dependencies);
            if let Some(import_map) = CONFIG.import_map {
                if import_map {
                    let imports = json!({ "imports": dependencies });
                    write(
                        format!("{}/import_map.json", &*DOTDIPLO),
                        serde_json::to_string(&imports)?,
                    )?;
                    extra_args.push(format!("--import-map={}/import_map.json", &*DOTDIPLO));
                }
            }
        }
        if let Some(load_env) = CONFIG.load_env {
            if load_env {
                if dotenv::dotenv().is_err() {
                    println!(
                        "{}",
                        format!("no .env file found continuing without loading dotenv").dimmed(),
                    );
                }
            }
        }

        if let Some(data) = CONFIG.scripts.as_ref().unwrap().get(script) {
            let mut tp = String::from("deno run ");

            //Allow inserting the import-map and future things
            tp.push_str(&extra_args.join(" "));

            let data_2 = data.replace("deno run", &tp);

            println!("Starting script {}", script.yellow());
            println!("> {}", data.dimmed());
            if sub_m.is_present("watch") {
                let config = get_config(&data_2);
                let handler = DiploHandler(ExecHandler::new(config)?);
                watch(&handler)?;
            } else {
                let mut parts = data_2.trim().split_whitespace();

                let command = parts.next().unwrap();

                let args = parts;

                let mut out = Command::new(command).args(args).spawn()?;

                if let Err(error) = out.wait() {
                    println!("{}", error);
                }
            }

            return Ok(());
        }
        println!(
            "Script not found please specify a script from the {} file",
            &*DIPLO_CONFIG.red()
        )
    }
    Ok(())
}
