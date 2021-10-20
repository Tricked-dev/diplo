use crate::DIPLO_CONFIG;
use anyhow::Result;
use clap::ArgMatches;
use colored::Colorize;
use serde_json::json;
use std::fs::{self};

pub fn exec(sub_m: &ArgMatches) -> Result<()> {
    if fs::File::open(&*DIPLO_CONFIG).is_ok() {
        let red = "THIS WILL RESET YOUR CONFIG".red();
        println!("{}", red);
    }
    //json option + yes enabled
    if sub_m.is_present("yes") && sub_m.is_present("json") {
        let data = json!({
            "name": "diplo-project",
            "load_env": false,
            "import_map": false,
            "dependencies": {},
            "scripts": {},
            "watcher": {}
        });

        fs::write("diplo.json", serde_json::to_string_pretty(&data)?)?;
    } else if sub_m.is_present("yes") {
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

        let data = if sub_m.is_present("json") {
            let data = json!({
                "name": name,
                "load_env":load_env,
                "import_map": import_map,
                "dependencies": {},
                "scripts": {},
                "watcher": {}
            });
            serde_json::to_string_pretty(&data)?
        } else {
            format!("name= \"{name}\"\nload_env={load_env}\nimport_map={import_map}\n[watcher]\n[dependencies]\n[scripts]",name=name,load_env=load_env, import_map = import_map )
        };
        if sub_m.is_present("json") {
            println!("Successfully wrote changes to {}", "diplo.json".green());
            fs::write("diplo.json", data)?;
        } else {
            println!("Successfully wrote changes to {}", &*DIPLO_CONFIG.green());
            fs::write(&*DIPLO_CONFIG, data)?;
        }
    }
    Ok(())
}
