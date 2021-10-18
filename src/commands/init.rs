use anyhow::Result;
use clap::ArgMatches;
use diplo::{info, term::print_inner, warn, DIPLOJSON};
use serde_json::json;
use std::fs::{self};

pub fn exec(sub_m: &ArgMatches) -> Result<()> {
    if fs::File::open(&*DIPLOJSON).is_ok() {
        warn!("THIS WILL RESET YOUR CONFIG");
    }

    if sub_m.is_present("yes") {
        let data = json!({
            "name": "diplo-project",
            "load_env": false,
            "import_map": false,
            "dependencies": {},
            "scripts": {},
            "watcher": {}
        });
        info!("Successfully wrote changes to {}", &*DIPLOJSON);
        fs::write(&*DIPLOJSON, serde_json::to_string_pretty(&data).unwrap()).unwrap();
    } else {
        let name = rprompt::prompt_reply_stderr("name : ").unwrap_or_else(|_| "".to_owned());
        let env =
            rprompt::prompt_reply_stderr("load_env (false): ").unwrap_or_else(|_| "".to_owned());

        let load_env = env.contains("true");

        let import =
            rprompt::prompt_reply_stderr("import_map (false): ").unwrap_or_else(|_| "".to_owned());

        let import_map = import.contains("true");

        let data = json!({
            "name": name,
            "load_env":load_env,
            "import_map": import_map,
            "dependencies": {},
            "scripts": {},
            "watcher": {}
        });
        info!("Succesfully wrote changes to {}", &*DIPLOJSON);
        fs::write(&*DIPLOJSON, serde_json::to_string_pretty(&data).unwrap()).unwrap();
    }
    Ok(())
}
