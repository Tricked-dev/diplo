use crate::app::create_app;
use crate::load_env::load_env;
use anyhow::Result;
use clap::ArgMatches;
use colored::Colorize;
use humantime::format_duration;
use lazy_static::lazy_static;
use std::{process, time::Instant};
lazy_static! {
    //Regex to replace the extra long formatting > 1ms 997us 241ns
    static ref REG: regex::Regex = regex::Regex::new("us (.*)ns").unwrap();
}

fn print_help() -> Result<()> {
    create_app().print_long_help().unwrap();
    Ok(())
}

pub fn run_default() -> Result<()> {
    let data = &run::cli().get_matches();
    let started = Instant::now();
    let print_results = move || {
        let now = Instant::now();
        let time = format_duration(now.duration_since(started)).to_string();
        let formatted_date = format!("{}us", REG.replace(&time, ""));
        println!();
        println!("{} Done in {}", ">".red(), formatted_date.green());
    };

    ctrlc::set_handler(move || {
        print_results();
        process::exit(101)
    })
    .unwrap_or_default();

    load_env(Some(data.is_present("load_env")));

    let result = run::exec(data);

    if result.is_ok() {
        print_results()
    } else if let Err(error) = result {
        println!("{}", "FATAL ERROR OCCURRED WHILE RUNNING SUBCOMMAND".red());
        println!("{}", format!("{:?}", error).dimmed());
        println!("{}", "PLEASE MAKE A ISSUE REPORTING THIS ERROR".red());
        println!("{}", "https://github.com/Tricked-dev/diplo".bright_red());
    }
    Ok(())
}

pub async fn handle_match(data: ArgMatches) -> Result<()> {
    let started = Instant::now();
    let print_results = move || {
        let now = Instant::now();
        let time = format_duration(now.duration_since(started)).to_string();
        let formatted_date = format!("{}us", REG.replace(&time, ""));
        println!();
        println!("{} Done in {}", ">".red(), formatted_date.green());
    };

    ctrlc::set_handler(move || {
        print_results();
        process::exit(101)
    })
    .unwrap_or_default();

    load_env(Some(data.is_present("load_env")));

    //Contributor note - please keep them alphabetically ordered
    let result = match data.subcommand() {
        Some(("add", args)) => add::exec(args).await,
        Some(("cache", _)) => cache::exec(),
        Some(("exec", args)) => exec::exec(args),
        Some(("init", args)) => init::exec(args),
        Some(("install", _)) => install::exec(),
        Some(("run", args)) => run::exec(args),
        Some(("update", _)) => update::exec().await,
        _ => print_help(),
    };
    if result.is_ok() {
        print_results()
    } else if let Err(error) = result {
        println!("{}", "FATAL ERROR OCCURRED WHILE RUNNING SUBCOMMAND".red());
        println!("{}", format!("{:?}", error).dimmed());
        println!("{}", "PLEASE MAKE A ISSUE REPORTING THIS ERROR".red());
        println!("{}", "https://github.com/Tricked-dev/diplo".bright_red());
        //TODO: add backtrace - https://github.com/rust-lang/rust/issues/53487
    }
    Ok(())
}

pub mod add;
pub mod cache;
pub mod exec;
pub mod init;
pub mod install;
pub mod run;
pub mod update;
