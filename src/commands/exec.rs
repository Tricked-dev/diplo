use crate::{
    command_prelude::*,
    run_utils::{append_extra_args, ensure_dependencies, run_script},
    utils::load_env,
    watcher::{get_config, DiploHandler},
    CONFIG,
};
use anyhow::Result;
use clap::ArgMatches;
use watchexec::{run::ExecHandler, watch};

pub fn cli() -> App<'static> {
    App::new("exec")
        .about("Dynamically run a command")
        .arg(
            Arg::new("command")
                .help("command to run")
                .required(true)
                .takes_value(true)
                .multiple_values(true),
        )
        .arg(
            Arg::new("load env")
                .help("Load the environment values using the rust dotenv crate")
                .required(false)
                .takes_value(false)
                .short('e')
                .long("load_env"),
        )
        .arg(
            Arg::new("watch")
                .help("Watch the filesystem for changes and restart on changes")
                .required(false)
                .takes_value(false)
                .short('w')
                .long("watch"),
        )
}

pub fn exec(sub_m: &ArgMatches) -> Result<()> {
    if let Some(script) = sub_m.values_of("command") {
        let extra_args: Vec<String> = vec![];

        ensure_dependencies()?;
        load_env::load_env(CONFIG.load_env);

        let command = append_extra_args(script.collect::<Vec<&str>>().join(" "), extra_args);

        if sub_m.is_present("watch") {
            let config = get_config(&command);
            let handler = DiploHandler(ExecHandler::new(config)?);
            watch(&handler).unwrap();
        } else {
            run_script(command)?;
        }
    }
    Ok(())
}
