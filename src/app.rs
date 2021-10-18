use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};

pub fn create_app() -> App<'static> {
    App::new(crate_name!())
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
            App::new("init")
                .about("Initialize diplo")
                .arg(
                    Arg::new("yes")
                        .about("Accept all options")
                        .required(false)
                        .takes_value(false)
                        .short('y')
                        .long("yes"),
                )
                .arg(
                    Arg::new("json")
                        .about("Create a config using the json format instead of toml")
                        .required(false)
                        .takes_value(false)
                        .short('j')
                        .long("json"),
                ),
        )
        .subcommand(App::new("cache").about("Cache the dependencies"))
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
                        .required(true)
                        .takes_value(true)
                        .multiple_values(true),
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
}
