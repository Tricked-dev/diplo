use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    env,
    fs::{create_dir_all, write},
};

lazy_static! {
    pub static ref DIPLOJSON: String = env::var("DIPLOJSON").unwrap_or("diplo.json".to_owned());
    pub static ref DOTDIPLO: String = env::var("DOTDIPLO").unwrap_or(".diplo".to_owned());
}

pub fn create_deps(dependencies: &HashMap<String, String>) {
    create_dir_all(&*DOTDIPLO).unwrap();
    let mut data: Vec<String> = vec![];
    for (_, value) in dependencies.iter() {
        data.push(format!("export * from \"{}\"", value))
    }
    write(format!("{}/deps.ts", &*DOTDIPLO), data.join("\n")).unwrap()
}
