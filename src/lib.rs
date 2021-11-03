pub mod app;
pub mod commands;
pub mod load_config;
pub mod update_deno;
mod utils;
pub mod watcher;
use lazy_static::lazy_static;
use load_config::{create_config, Config};
use once_cell::sync::Lazy;
use std::env;
use utils::*;

lazy_static! {
    pub static ref DIPLO_CONFIG: String = init_config();
    pub static ref DOTDIPLO: String = env::var("DOTDIPLO").unwrap_or_else(|_| ".diplo".to_owned());
    pub static ref CONFIG: Lazy<Config> = Lazy::new(create_config);
}

fn init_config() -> String {
    let config = env::var("DIPLO_CONFIG");
    if let Ok(config) = config {
        config
    } else {
        "diplo.toml".to_owned()
    }
}
pub mod command_prelude {
    pub use clap::{App, Arg, ArgMatches};
}
