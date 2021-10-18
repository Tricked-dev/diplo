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

pub fn exec() -> Result<()> {
    if let Some(dependencies) = &CONFIG.dependencies {
        create_deps(dependencies);
        if let Some(import_map) = CONFIG.import_map {
            if import_map {
                let imports = json!({ "imports": dependencies });
                write(
                    format!("{}/import_map.json", &*DOTDIPLO),
                    serde_json::to_string(&imports).unwrap(),
                )
                .unwrap();
            }
        }
    }
    info!("Successfully initialized diplo");

    Ok(())
}
