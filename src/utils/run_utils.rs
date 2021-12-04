use crate::{load_config::Dependency, CONFIG, DOTDIPLO};
use anyhow::Result;
use serde_json::json;
use std::{
    collections::HashMap,
    fs::{create_dir_all, write},
    process::Command,
};

pub fn get_dep_urls() -> HashMap<String, String> {
    let mut res = HashMap::new();
    if let Some(deps) = CONFIG.dependencies.as_ref() {
        for (key, val) in deps.iter() {
            res.insert(key.to_owned(), val.url.clone());
        }
    }

    res
}

pub fn create_deps(dependencies: &HashMap<String, Dependency>) {
    create_dir_all(&*DOTDIPLO).unwrap();
    let mut data: Vec<String> = vec![];

    for (_key, value) in dependencies.iter() {
        let export = if let Some(data) = &value.exports {
            if data.contains('*') || data.contains('{') {
                data.to_owned()
            } else {
                format!("{{ {} }}", data)
            }
        } else {
            "*".to_owned()
        };
        data.push(format!(
            "{}export {} from \"{}\"",
            if let Some(types) = &value.types {
                format!("// @deno-types=\"{}\"\n", types)
            } else {
                "".to_owned()
            },
            export,
            value.url
        ))
    }
    data.sort();
    write(format!("{}/deps.ts", &*DOTDIPLO), data.join("\n")).unwrap()
}

pub fn ensure_dependencies() -> Result<()> {
    if let Some(dependencies) = &CONFIG.dependencies {
        create_deps(dependencies);
        let import_map = get_dep_urls();

        let imports = json!({ "imports": import_map });
        write(
            format!("{}/import_map.json", &*DOTDIPLO),
            serde_json::to_string(&imports)?,
        )?;
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
