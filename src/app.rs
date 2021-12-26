use crate::commands::*;
use clap::{crate_authors, crate_description, crate_name, crate_version, App};
use clap::AppSettings::AllowExternalSubcommands;

pub fn create_app() -> App<'static> {
    //Contributor note - please keep them alphabetically ordered
    App::new(crate_name!())
        .setting(AllowExternalSubcommands)
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(add::cli())
        .subcommand(cache::cli())
        .subcommand(exec::cli())
        .subcommand(init::cli())
        .subcommand(install::cli())
        .subcommand(run::cli())
        .subcommand(update::cli())
}
