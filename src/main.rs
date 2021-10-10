use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use diplo::{
    create_deps, update_config,
    update_deno::{update_deps, Versions, HTTP_CLIENT},
    DIPLOJSON, DOTDIPLO,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    collections::HashMap,
    env,
    fs::{self, read_to_string, write},
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = read_to_string(&*DIPLOJSON);

    let mut config: Config = Config {
        load_env: Some(false),
        import_map: Some(false),
        name: None,
        scripts: Some(HashMap::new()),
        dependencies: Some(HashMap::new()),
    };
    let has_config: bool;
    if let Ok(data) = data {
        config = serde_json::from_str(&data).unwrap();
        has_config = true
    } else {
        has_config = false
    }

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
                // TODO: add watch its way too hard to add with closures etc but maybe one day
                // .arg(
                //     Arg::new("watch")
                //         .about("Watch the filesystem for changes and restart on changes")
                //         .required(false)
                //         .takes_value(false)
                //         .short('w')
                //         .long("watch"),
                // ),
        ).subcommand(App::new("init").about(
            "Initialize diplo",
        )  .arg(
                    Arg::new("yes")
                        .about("Accept all options")
                        .required(false)
                        .takes_value(false)
                        .short('y')
                        .long("yes"),
                ),)
        .subcommand(App::new("install").about(
            "This creates the .diplo directory with all required files",
        )).subcommand(App::new("update").about(
            "This updates all deno.land/x/ modules to their latest version",
        )).subcommand(App::new("add").about(
            "Add a deno.land/x/ module",
        ).arg(
                    Arg::new("module")
                        .about("Deno module you want to add")
                        .required(true),
                ))
        .get_matches();

    match matches.subcommand() {
        Some(("run", sub_m)) => {
            if let Some(script) = sub_m.value_of("script") {
                let mut extra_args: Vec<String> = vec![];

                if let Some(dependencies) = config.dependencies {
                    create_deps(&dependencies);
                    if let Some(import_map) = config.import_map {
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
                if let Some(load_env) = config.load_env {
                    if load_env {
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
                    return Ok(());
                }
                println!(
                    "Script not found please specify a script from the {} file",
                    &*DIPLOJSON
                )
            }
        }
        Some(("init", sub_m)) => {
            if sub_m.is_present("yes") {
                let data = json!({
                    "name": "diplo-project",
                    "load_env":false,
                    "import_map": false,
                    "dependencies": {}
                });
                println!("Succesfully wrote changes to {}", &*DIPLOJSON);
                fs::write(&*DIPLOJSON, serde_json::to_string_pretty(&data).unwrap()).unwrap();
            } else {
                if has_config {
                    println!("WARNING THIS WILL OVERWRITE YOUR OLD {} FILE", &*DIPLOJSON)
                }
                let name =
                    rprompt::prompt_reply_stderr("name : ").unwrap_or_else(|_| "".to_owned());
                let env = rprompt::prompt_reply_stderr("load_env (false): ")
                    .unwrap_or_else(|_| "".to_owned());
                let load_env: bool;
                //TODO: FIX THIS MESSY CODE
                if env.contains("true") {
                    load_env = true
                } else {
                    load_env = false
                };
                let import = rprompt::prompt_reply_stderr("import_map (false): ")
                    .unwrap_or_else(|_| "".to_owned());
                let import_map: bool;
                //TODO: FIX THIS MESSY CODE
                if import.contains("true") {
                    import_map = true
                } else {
                    import_map = false
                };
                let data = json!({
                    "name": name,
                    "load_env":load_env,
                    "import_map": import_map,
                    "dependencies": {}
                });
                println!("Succesfully wrote changes to {}", &*DIPLOJSON);
                fs::write(&*DIPLOJSON, serde_json::to_string_pretty(&data).unwrap()).unwrap();
            }
        }
        Some(("add", sub_m)) => {
            if let Some(module) = sub_m.value_of("module") {
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
                    let mut deps = config.dependencies.unwrap();
                    deps.insert(
                        (&module).to_string(),
                        format!("https://deno.land/x/{}@{}/mod.ts", module, json.latest),
                    );
                    update_config(json!({ "dependencies": deps }));
                    println!(
                        "Succesfully added {}@{} to the depdencies",
                        module, json.latest
                    )
                } else {
                    println!("No module named {} found", module)
                }
            }
        }
        Some(("install", _)) => {
            if let Some(dependencies) = config.dependencies {
                create_deps(&dependencies);
                if let Some(import_map) = config.import_map {
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
            println!("Successfully initialized diplo")
        }
        Some(("update", _)) => {
            let newdeps = update_deps(&config.dependencies.unwrap()).await;
            if let true = update_config(json!({ "dependencies": &newdeps })) {
                println!("updating done!");
            }

            // let data = read_to_string(&*DIPLOJSON);
            // if let Ok(data) = data {
            //     let mut config: Value = serde_json::from_str(&data).unwrap();
            //     merge(&mut config, json!({ "dependencies": &newdeps }));

            //     fs::write(&*DIPLOJSON, serde_json::to_string_pretty(&config).unwrap()).unwrap();
            //     println!("updating done!")
            // } else {
            //     println!("No diplo.json file found");
            // }
        }
        _ => println!("INVALID ARGUMENT USE --help FOR ALL COMMANDS"), // commit was used                       // Either no subcommand or one not tested for...
    }
    Ok(())
}
