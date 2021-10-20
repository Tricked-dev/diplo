use crate::{DIPLO_CONFIG, DOTDIPLO};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use std::{
    collections::HashMap,
    fs::{create_dir_all, read_to_string, write},
};
use toml_edit::Document;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub name: Option<String>,
    pub scripts: Option<HashMap<String, String>>,
    pub load_env: Option<bool>,
    pub import_map: Option<bool>,
    pub dependencies: Option<HashMap<String, String>>,
    pub watcher: Option<WatcherClass>,
}

#[derive(Serialize, Deserialize)]
pub struct WatcherClass {
    pub directory: Option<String>,
    pub default_ignores: Option<bool>,
    pub clear: Option<bool>,
    pub no_ignore: Option<bool>,
    pub respect_gitignore: Option<bool>,
}

pub fn create_config() -> Config {
    create_dir_all(&*DOTDIPLO).unwrap();
    let data = read_to_string(&*DIPLO_CONFIG);

    let config: Config = match data {
        Ok(data) =>
        //if DIPLO_CONFIG.ends_with(".json")
        {
            if DIPLO_CONFIG.ends_with(".toml") {
                toml::from_str(&data).unwrap()
            } else {
                serde_json::from_str(&data).unwrap()
            }
        }
        // serde_json::from_str(&data).unwrap()
        _ => Config {
            load_env: Some(false),
            import_map: Some(false),
            name: None,
            scripts: Some(HashMap::new()),
            dependencies: Some(HashMap::new()),
            watcher: None,
        },
    };

    // let mut config: Config = Config {
    //     load_env: Some(false),
    //     import_map: Some(false),
    //     name: None,
    //     scripts: Some(HashMap::new()),
    //     dependencies: Some(HashMap::new()),
    //     watcher: None,
    // };

    // if let Ok(data) = data {
    //     config = serde_json::from_str(&data).unwrap();
    // }

    config
}

pub fn merge(a: &mut Value, b: Value) {
    match (a, b) {
        (a @ &mut Value::Object(_), Value::Object(b)) => {
            let a = a.as_object_mut().unwrap();
            for (k, v) in b {
                merge(a.entry(k).or_insert(Value::Null), v);
            }
        }
        (a, b) => *a = b,
    }
}

pub fn update_config_toml(config: Document) {
    write(&*DIPLO_CONFIG, config.to_string()).unwrap();
}

pub fn update_config_json(val: Value) -> bool {
    let data = read_to_string(&*DIPLO_CONFIG);
    if let Ok(data) = data {
        let mut data: Value = serde_json::from_str(&data).unwrap_or_else(|_| json!({}));
        merge(&mut data, val);

        write(&*DIPLO_CONFIG, serde_json::to_string_pretty(&data).unwrap()).unwrap();
        true
    } else {
        println!(
            "No {} file found please create one or run diplo init",
            &*DIPLO_CONFIG.red()
        );
        false
    }
}
