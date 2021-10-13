use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
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
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(
            App::new("run")
                .about("Run a diplo script")
                .arg(
                    Arg::new("script")
                        .about("The script to run defined in the diplo.json file")
                        .required(true),
                )
                .arg(
                    Arg::new("watch")
                        .about("Watch the filesystem for changes and restart on changes")
                        .required(false)
                        .takes_value(false)
                        .short('w')
                        .long("watch"),
                ),
        )
        .subcommand(
            App::new("init").about("Initialize diplo").arg(
                Arg::new("yes")
                    .about("Accept all options")
                    .required(false)
                    .takes_value(false)
                    .short('y')
                    .long("yes"),
            ),
        )
        .subcommand(
            App::new("exec")
                .about("Dynamically run a command")
                .arg(Arg::new("command").about("command to run").required(true))
                .arg(
                    Arg::new("watch")
                        .about("Watch the filesystem for changes and restart on changes")
                        .required(false)
                        .takes_value(false)
                        .short('w')
                        .long("watch"),
                ),
        )
        .subcommand(
            App::new("install").about("This creates the .diplo directory with all required files"),
        )
        .subcommand(
            App::new("update")
                .about("This updates all deno.land/x/ modules to their latest version"),
        )
        .subcommand(
            App::new("add")
                .about("Add a deno.land/x/ module")
                .arg(
                    Arg::new("module")
                        .about("Deno module you want to add")
                        .required(true),
                )
                .arg(
                    Arg::new("std")
                        .about("Add a std package")
                        .required(false)
                        .takes_value(false)
                        .short('s')
                        .long("std"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("run", sub_m)) => {
            if let Some(script) = sub_m.value_of("script") {
                let mut extra_args: Vec<String> = vec![];

                if let Some(dependencies) = &CONFIG.dependencies {
                    create_deps(dependencies);
                    if let Some(import_map) = CONFIG.import_map {
                        if import_map {
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
                if let Some(load_env) = CONFIG.load_env {
                    if load_env {
                        dotenv::dotenv().expect("COULD NOT FIND .env FILE IN CURRENT DIRECTORY");
                    }
                }

                if let Some(data) = CONFIG.scripts.as_ref().unwrap().get(script) {
                    let mut tp = String::from("deno run ");

                    //Allow inserting the import-map and future things
                    tp.push_str(&extra_args.join(" "));

                    let data_2 = data.replace("deno run", &tp);

                    if sub_m.is_present("watch") {
                        let config = get_config(&data_2);
                        let handler = DiploHandler(ExecHandler::new(config)?);
                        watch(&handler).unwrap();
                    } else {
                        let mut parts = data_2.trim().split_whitespace();

                        let command = parts.next().unwrap();

                        let args = parts;

                        let mut out = Command::new(command).args(args).spawn().unwrap();

                        if let Err(error) = out.wait() {
                            println!("{}", error);
                        }
                    }

                    return Ok(());
                }
                warn!(
                    "Script not found please specify a script from the {} file",
                    &*DIPLOJSON
                )
            }
        }
        Some(("exec", sub_m)) => {
            if let Some(script) = sub_m.value_of("command") {
                let mut extra_args: Vec<String> = vec![];

                if let Some(dependencies) = &CONFIG.dependencies {
                    create_deps(dependencies);
                    if let Some(import_map) = CONFIG.import_map {
                        if import_map {
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
                if let Some(load_env) = CONFIG.load_env {
                    if load_env {
                        dotenv::dotenv().expect("COULD NOT FIND .env FILE IN CURRENT DIRECTORY");
                    }
                }

                let mut tp = String::from("deno run ");

                //Allow inserting the import-map and future things
                tp.push_str(&extra_args.join(" "));

                let data_2 = script.replace("deno run", &tp);

                if sub_m.is_present("watch") {
                    let config = get_config(&data_2);
                    let handler = DiploHandler(ExecHandler::new(config)?);
                    watch(&handler).unwrap();
                } else {
                    let mut parts = data_2.trim().split_whitespace();

                    let command = parts.next().unwrap();

                    let args = parts;

                    let mut out = Command::new(command).args(args).spawn().unwrap();

                    if let Err(error) = out.wait() {
                        println!("{}", error);
                    }
                }
            }
        }

        Some(("init", sub_m)) => {
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
                let name =
                    rprompt::prompt_reply_stderr("name : ").unwrap_or_else(|_| "".to_owned());
                let env = rprompt::prompt_reply_stderr("load_env (false): ")
                    .unwrap_or_else(|_| "".to_owned());

                let load_env = if env.contains("true") { true } else { false };

                let import = rprompt::prompt_reply_stderr("import_map (false): ")
                    .unwrap_or_else(|_| "".to_owned());

                let import_map = if import.contains("true") { true } else { false };

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
        }
        Some(("add", sub_m)) => {
            if let Some(module) = sub_m.value_of("module") {
                if sub_m.is_present("std") {
                    let latest_std = get_latest_std().await;

                    let data = &format!("https://deno.land/std@{}/{}/mod.ts", latest_std, module);
                    let mut deps = CONFIG.dependencies.as_ref().unwrap().clone();
                    deps.insert((&module).to_string(), data.to_string());
                    //Errors otherwise
                    if let true = update_config(json!({ "dependencies": deps })) {
                        info!("Succesfully added {} to the dependencies", data)
                    }
                } else {
                    let res = HTTP_CLIENT
                        .get(format!(
                            "https://cdn.deno.land/{}/meta/versions.json",
                            &module
                        ))
                        .header("user-agent", "diplo")
                        .send()
                        .await
                        .unwrap();
                    let text = res.text().await.unwrap();

                    let json: Result<Versions, serde_json::Error> = serde_json::from_str(&text);
                    if let Ok(json) = json {
                        let mut deps = CONFIG.dependencies.as_ref().unwrap().clone();
                        deps.insert(
                            (&module).to_string(),
                            format!("https://deno.land/x/{}@{}/mod.ts", module, json.latest),
                        );
                        //Errors otherwise
                        if let true = update_config(json!({ "dependencies": deps })) {
                            info!(
                                "Succesfully added {}@{} to the dependencies",
                                module, json.latest
                            )
                        }
                    } else {
                        info!("No module named {} found", module)
                    }
                }
            }
        }
        Some(("install", _)) => {
            if let Some(dependencies) = &CONFIG.dependencies {
                create_deps(dependencies);
                if let Some(import_map) = CONFIG.import_map {
                    if import_map {
                        let imports = json!({ "imports": dependencies });
                        write(
                            format!("{}/import_map.json", &*DOTDIPLO),
                            serde_json::to_string(&imports).unwrap(),
                        )
                        .unwrap();
                    }
                }
            }
            info!("Successfully initialized diplo")
        }
        Some(("update", _)) => {
            let newdeps = update_deps(CONFIG.dependencies.as_ref().unwrap()).await;
            if let true = update_config(json!({ "dependencies": &newdeps })) {
                info!("updating done!");
            }
        }
        _ => error!("INVALID ARGUMENT USE --help FOR ALL COMMANDS"),
    }
    Ok(())
}
