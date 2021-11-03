use crate::{DIPLO_CONFIG, DOTDIPLO};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::HashMap,
    fs::{create_dir_all, read_to_string, write},
};
use toml_edit::Document;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Dependency {
    pub url: String,
    pub exports: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub name: Option<String>,
    pub scripts: Option<HashMap<String, String>>,
    pub load_env: Option<bool>,
    pub import_map: Option<bool>,
    pub dependencies: Option<HashMap<String, Dependency>>,
    pub watcher: Option<WatcherClass>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct ConfigParse {
    pub name: Option<String>,
    pub scripts: Option<HashMap<String, String>>,
    pub load_env: Option<bool>,
    pub import_map: Option<bool>,
    pub dependencies: Option<HashMap<String, Value>>,
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

    let config: ConfigParse = match data {
        Ok(data) => toml::from_str(&data).unwrap(),
        _ => ConfigParse {
            load_env: Some(false),
            import_map: Some(false),
            name: None,
            scripts: Some(HashMap::new()),
            dependencies: Some(HashMap::new()),
            watcher: None,
        },
    };
    let mut new_deps: HashMap<String, Dependency> = HashMap::new();
    if let Some(deps) = config.dependencies {
        for (key, val) in deps.iter() {
            if val.is_object() {
                new_deps.insert(
                    key.to_string(),
                    Dependency {
                        url: val["url"].as_str().unwrap().to_owned(),
                        exports: val["exports"].as_str().map(|exports| exports.to_string()),
                    },
                );
            } else {
                new_deps.insert(
                    key.to_string(),
                    Dependency {
                        url: val.as_str().unwrap().to_owned(),
                        exports: None,
                    },
                );
            }
        }
    }
    Config {
        name: config.name,
        scripts: config.scripts,
        load_env: config.load_env,
        import_map: config.import_map,
        dependencies: Some(new_deps),
        watcher: config.watcher,
    }
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
