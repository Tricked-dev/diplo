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

pub async fn handle_match(data: ArgMatches) -> Result<()> {
    match data.subcommand() {
        Some(("add", sub_m)) => add::exec(Box::new(sub_m)).await.unwrap(),
        Some(("exec", sub_m)) => exec::exec(Box::new(sub_m)).unwrap(),
        Some(("init", sub_m)) => init::exec(Box::new(sub_m)).unwrap(),
        Some(("install", _)) => install::exec().unwrap(),
        Some(("run", sub_m)) => run::exec(Box::new(sub_m)).unwrap(),
        Some(("update", _)) => update::exec().await.unwrap(),
        _ => error!("INVALID ARGUMENT USE --help FOR ALL COMMANDS"),
    };
    Ok(())
}

mod add;
mod exec;
mod init;
mod install;
mod run;
mod update;
