use crate::app::create_app;
use anyhow::Result;
use clap::ArgMatches;

pub async fn handle_match(data: ArgMatches) -> Result<()> {
    match data.subcommand() {
        Some(("add", sub_m)) => add::exec(sub_m).await.unwrap(),
        Some(("cache", _)) => cache::exec().unwrap(),
        Some(("exec", sub_m)) => exec::exec(sub_m).unwrap(),
        Some(("init", sub_m)) => init::exec(sub_m).unwrap(),
        Some(("install", _)) => install::exec().unwrap(),
        Some(("run", sub_m)) => run::exec(sub_m).unwrap(),
        Some(("update", _)) => update::exec().await.unwrap(),
        _ => create_app().print_long_help().unwrap(),
    };
    Ok(())
}

mod add;
mod cache;
mod exec;
mod init;
mod install;
mod run;
mod update;
