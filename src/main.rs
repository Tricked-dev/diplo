use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use diplo::{create_deps, DIPLOJSON, DOTDIPLO};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    collections::HashMap,
    env,
    fs::{read_to_string, write},
    process::Command,
};

#[derive(Serialize, Deserialize)]
pub struct Config {
    name: Option<String>,
    scripts: Option<HashMap<String, String>>,
    load_env: Option<bool>,
    import_map: Option<bool>,
    dependencies: Option<HashMap<String, String>>,
}

fn main() {
    let data = read_to_string(&*DIPLOJSON);
    let mut config: Config = Config {
        load_env: Some(false),
        import_map: Some(false),
        name: Some("".to_string()),
        scripts: Some(HashMap::new()),
        dependencies: Some(HashMap::new()),
    };

    if let Ok(data) = data {
        config = serde_json::from_str(&data).unwrap();
    }

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(
            App::new("run").about("run a script").arg(
                Arg::new("script")
                    .about("The script to run defined in the diplo.json file")
                    .required(true),
            ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("run", sub_m)) => {
            if let Some(script) = sub_m.value_of("script") {
                let mut extra_args: Vec<String> = vec![];

                if let Some(dependencies) = config.dependencies {
                    create_deps(&dependencies);
                    if let Some(import_map) = config.import_map {
                        if import_map == true {
                            let imports = json!({ "imports": dependencies });
                            write(
                                format!("{}/import_map.json", &*DOTDIPLO),
                                serde_json::to_string(&imports).unwrap(),
                            )
                            .unwrap();
                            extra_args.push(format!("--import-map={}/import_map.json", &*DOTDIPLO));
                        }
                    }
                }
                if let Some(load_env) = config.load_env {
                    if load_env == true {
                        dotenv::dotenv().expect("COULD NOT FIND .env FILE IN CURRENT DIRECTORY");
                    }
                }

                if let Some(data) = config.scripts.unwrap().get(script) {
                    let mut tp = String::from("deno run ");

                    //Allow inserting the import-map and future things
                    tp.push_str(&extra_args.join(" "));

                    let data_2 = data.replace("deno run", &tp);

                    let mut parts = data_2.trim().split_whitespace();

                    let command = parts.next().unwrap();

                    let args = parts;

                    let mut out = Command::new(command).args(args).spawn().unwrap();
                    if let Err(error) = out.wait() {
                        println!("{}", error);
                    }
                    return;
                }
                println!(
                    "Script not found please specify a script from the {} file",
                    &*DIPLOJSON
                )
            }
        } // clone was used

        _ => println!("INVALID ARGUMENT USE --help FOR ALL COMMANDS"), // commit was used                       // Either no subcommand or one not tested for...
    }
}
