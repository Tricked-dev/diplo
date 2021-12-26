use crate::{command_prelude::*, DIPLO_CONFIG};
use anyhow::Result;
use clap::ArgMatches;
use colored::Colorize;
use std::fs;

pub fn cli() -> App<'static> {
    App::new("init")
                .about("Initialize diplo")
                .arg(
                    Arg::new("yes")
                        .help("Accept all options")
                        .required(false)
                        .takes_value(false)
                        .short('y')
                        .long("yes"),
                )
                .arg(
                    Arg::new("json")
                        .help("Create a config using the json format instead of toml")
                        .long_help("Create a config using the json format instead of toml\nThis is not recommended to do due to diplo being build with toml in mind")
                        .required(false)
                        .takes_value(false)
                        .short('j')
                        .long("json"),
                )
}

pub fn exec(sub_m: &ArgMatches) -> Result<()> {
    if fs::File::open(&*DIPLO_CONFIG).is_ok() {
        let red = "THIS WILL RESET YOUR CONFIG".red();
        println!("{}", red);
    }

    if sub_m.is_present("yes") {
        let data = "name= \"diplo project\"\nload_env=false\nimport_map=false\n[dependencies]\n[watcher]\n[scripts]";
        println!("Successfully wrote changes to {}", &*DIPLO_CONFIG.green());
        fs::write(&*DIPLO_CONFIG, data)?;
    } else {
        let name = rprompt::prompt_reply_stderr("name : ").unwrap_or_else(|_| "".to_owned());
        let env =
            rprompt::prompt_reply_stderr("load_env (false): ").unwrap_or_else(|_| "".to_owned());

        let load_env = env.contains("true");

        let import =
            rprompt::prompt_reply_stderr("import_map (false): ").unwrap_or_else(|_| "".to_owned());

        let import_map = import.contains("true");

        let data =
            format!("name= \"{name}\"\nload_env={load_env}\nimport_map={import_map}\n[watcher]\n[dependencies]\n[scripts]",name=name,load_env=load_env, import_map = import_map );

        println!("Successfully wrote changes to {}", &*DIPLO_CONFIG.green());
        fs::write(&*DIPLO_CONFIG, data)?;
    }
    Ok(())
}
