use crate::{CONFIG, DOTDIPLO};
use anyhow::Result;
use serde_json::json;
use std::{
    collections::HashMap,
    fs::{create_dir_all, write},
    process::Command,
};

pub fn create_deps(dependencies: &HashMap<String, String>) {
    create_dir_all(&*DOTDIPLO).unwrap();
    let mut data: Vec<String> = vec![];
    let empty_hashmap = HashMap::new();
    let exports = CONFIG.exports.as_ref().unwrap_or(&empty_hashmap);
    for (key, value) in dependencies.iter() {
        let export = if let Some(data) = exports.get(key) {
            if data.contains('*') || data.contains('{') {
                data.to_owned()
            } else {
                format!("{{ {} }}", data)
            }
        } else {
            "*".to_owned()
        };
        data.push(format!("export {} from \"{}\"", export, value))
    }
    data.sort();
    write(format!("{}/deps.ts", &*DOTDIPLO), data.join("\n")).unwrap()
}

pub fn ensure_dependencies() -> Result<()> {
    if let Some(dependencies) = &CONFIG.dependencies {
        create_deps(dependencies);
        if let Some(import_map) = CONFIG.import_map {
            if import_map {
                let imports = json!({ "imports": dependencies });
                write(
                    format!("{}/import_map.json", &*DOTDIPLO),
                    serde_json::to_string(&imports)?,
                )?;
            }
        }
    }
    Ok(())
}

pub fn append_extra_args(input: String, extra_args: Vec<String>) -> String {
    let mut data = "deno run ".to_owned();
    //Allow inserting the import-map and future things
    data.push_str(&extra_args.join(" "));
    input.replace("deno run", &data)
}

pub fn run_script(command: String) -> Result<()> {
    let mut parts = command.trim().split_whitespace();

    let command = parts.next().unwrap();

    let args = parts;

    let mut out = Command::new(command).args(args).spawn()?;

    if let Err(error) = out.wait() {
        println!("{}", error);
    }
    Ok(())
}
