pub mod update_deno;
use lazy_static::lazy_static;
use serde_json::{json, Value};
use std::{
    collections::HashMap,
    env,
    fs::{create_dir_all, read_to_string, write},
};

lazy_static! {
    pub static ref DIPLOJSON: String =
        env::var("DIPLOJSON").unwrap_or_else(|_| "diplo.json".to_owned());
    pub static ref DOTDIPLO: String = env::var("DOTDIPLO").unwrap_or_else(|_| ".diplo".to_owned());
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
        let mut config: Value = serde_json::from_str(&data).unwrap_or_else(|_| json!({}));
        merge(&mut config, val);

        write(&*DIPLOJSON, serde_json::to_string_pretty(&config).unwrap()).unwrap();
        true
    } else {
        println!("No {} file found please create one", &*DIPLOJSON);
        false
    }
}
