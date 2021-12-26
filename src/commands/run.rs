use crate::{
    command_prelude::*,
    load_env,
    run_utils::{append_extra_args, ensure_dependencies, run_script},
    watcher::{get_config, DiploHandler},
    CONFIG, DIPLO_CONFIG,
};
use anyhow::Result;
use colored::Colorize;
use watchexec::{run::ExecHandler, watch};

pub fn cli() -> App<'static> {
    App::new("run")
        .about("Run a diplo script")
        .arg(
            Arg::new("script")
                .help("The script to run defined in the diplo.json file")
                .required(true),
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
    if let Some(script) = sub_m.value_of("script") {
        let extra_args: Vec<String> = vec![];

        ensure_dependencies()?;
        load_env::load_env(CONFIG.load_env);

        if let Some(data) = CONFIG.scripts.as_ref().unwrap().get(script) {
            let data_2 = append_extra_args(data.to_string(), extra_args);
            println!("Starting script {}", script.yellow());
            println!("> {}", data.dimmed());
            if sub_m.is_present("watch") {
                let config = get_config(&data_2);
                let handler = DiploHandler(ExecHandler::new(config)?);
                watch(&handler)?;
            } else {
                run_script(data_2)?;
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
