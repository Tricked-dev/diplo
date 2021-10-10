use serde::{Deserialize, Serialize};

use crate::DIPLOJSON;
use std::{collections::HashMap, fs::read_to_string};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub name: Option<String>,
    pub scripts: Option<HashMap<String, String>>,
    pub load_env: Option<bool>,
    pub import_map: Option<bool>,
    pub dependencies: Option<HashMap<String, String>>,
}

pub fn create_config() -> Config {
    let data = read_to_string(&*DIPLOJSON);

    let mut config: Config = Config {
        load_env: Some(false),
        import_map: Some(false),
        name: None,
        scripts: Some(HashMap::new()),
        dependencies: Some(HashMap::new()),
    };

    if let Ok(data) = data {
        config = serde_json::from_str(&data).unwrap();
    }

    return config;
}
