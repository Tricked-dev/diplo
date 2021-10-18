mod app;
mod commands;
use app::create_app;
use commands::handle_match;
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
    env,
    fs::{self, write},
    process::Command,
};
use watchexec::{run::ExecHandler, watch};
// use watchexec::config::ConfigBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = create_app().get_matches();

    handle_match(matches).await.unwrap();

    Ok(())
}
