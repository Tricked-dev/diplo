use serde::{Deserialize, Serialize};

use crate::{error, term::print_inner, DIPLOJSON, DOTDIPLO};
use serde_json::json;
use serde_json::Value;
use std::{
    collections::HashMap,
    fs::{create_dir_all, read_to_string, write},
};

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
    let data = read_to_string(&*DIPLOJSON);

    let mut config: Config = Config {
        load_env: Some(false),
        import_map: Some(false),
        name: None,
        scripts: Some(HashMap::new()),
        dependencies: Some(HashMap::new()),
        watcher: None,
    };

    if let Ok(data) = data {
        config = serde_json::from_str(&data).unwrap();
    }

    config
}

pub fn create_deps(dependencies: &HashMap<String, String>) {
    create_dir_all(&*DOTDIPLO).unwrap();
    let mut data: Vec<String> = vec![];
    for (_, value) in dependencies.iter() {
        data.push(format!("export * from \"{}\"", value))
    }
    write(format!("{}/deps.ts", &*DOTDIPLO), data.join("\n")).unwrap()
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

pub fn update_config(val: Value) -> bool {
    let data = read_to_string(&*DIPLOJSON);
    if let Ok(data) = data {
        let mut data: Value = serde_json::from_str(&data).unwrap_or_else(|_| json!({}));
        merge(&mut data, val);

        write(&*DIPLOJSON, serde_json::to_string_pretty(&data).unwrap()).unwrap();
        true
    } else {
        error!(
            "No {} file found please create one or run diplo init",
            &*DIPLOJSON
        );
        false
    }
}
