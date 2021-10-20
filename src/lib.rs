pub mod app;
pub mod commands;
pub mod load_config;
pub mod update_deno;
pub mod watcher;
use lazy_static::lazy_static;
use load_config::{create_config, Config};
use once_cell::sync::Lazy;
use std::env;

//TODO: use try_exists when released https://github.com/rust-lang/rust/issues/83186
use std::fs::read;

lazy_static! {
    pub static ref DIPLO_CONFIG: String = init_config();
    pub static ref DOTDIPLO: String = env::var("DOTDIPLO").unwrap_or_else(|_| ".diplo".to_owned());
    pub static ref CONFIG: Lazy<Config> = Lazy::new(create_config);
}

fn init_config() -> String {
    let config = env::var("DIPLO_CONFIG");
    if let Ok(config) = config {
        config
    } else if read("diplo.json").is_ok() {
        "diplo.json".to_owned()
    } else if read("diplo.toml").is_ok() {
        "diplo.toml".to_owned()
    } else if read("diplo.yml").is_ok() {
        unimplemented!("Diplo.yml isn't supported yet!");
        // "diplo.yml".to_owned()
    } else {
        "diplo.toml".to_owned()
    }
}
