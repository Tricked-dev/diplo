use crate::app::create_app;
use anyhow::Result;
use clap::ArgMatches;
use colored::Colorize;
use humantime::format_duration;
use lazy_static::lazy_static;
use std::time::Instant;
lazy_static! {
    //Regex to replace the extra long formatting > 1ms 997us 241ns
    static ref REG: regex::Regex = regex::Regex::new("us (.*)ns").unwrap();
}

pub async fn handle_match(data: ArgMatches) -> Result<()> {
    let started = Instant::now();
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
    let now = Instant::now();
    let time = format_duration(now.duration_since(started)).to_string();
    let formatted_date = format!("{}us", REG.replace(&time, ""));
    println!("{} Done in {}", ">".red(), formatted_date.green());
    Ok(())
}

mod add;
mod cache;
mod exec;
mod init;
mod install;
mod run;
mod update;
