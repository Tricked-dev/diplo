pub mod load_config;
pub mod update_deno;
pub mod watcher;
use lazy_static::lazy_static;
use load_config::{create_config, Config};
use std::env;

lazy_static! {
    pub static ref DIPLOJSON: String =
        env::var("DIPLOJSON").unwrap_or_else(|_| "diplo.json".to_owned());
    pub static ref DOTDIPLO: String = env::var("DOTDIPLO").unwrap_or_else(|_| ".diplo".to_owned());
    pub static ref CONFIG: Config = create_config();
}
