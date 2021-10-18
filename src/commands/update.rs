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

pub async fn exec() -> Result<()> {
    let newdeps = update_deps(CONFIG.dependencies.as_ref().unwrap()).await;
    if let true = update_config(json!({ "dependencies": &newdeps })) {
        info!("updating done!");
    }
    Ok(())
}
