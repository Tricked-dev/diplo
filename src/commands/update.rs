use crate::{
    load_config::{update_config_json, update_config_toml},
    update_deno::update_deps,
    CONFIG, DIPLO_CONFIG,
};
use anyhow::Result;
use serde_json::json;
use std::fs::read_to_string;
use toml_edit::{value, Document};

pub async fn exec() -> Result<()> {
    let newdeps = update_deps(CONFIG.dependencies.as_ref().unwrap()).await;
    if DIPLO_CONFIG.ends_with(".toml") {
        //Cant error cause it would default to json
        let data = read_to_string(&*DIPLO_CONFIG)?;
        let mut document = data.parse::<Document>()?;
        for (name, val) in newdeps.iter() {
            document["dependencies"][name] = value(val);
        }
        update_config_toml(document);
    } else if let true = update_config_json(json!({ "dependencies": &newdeps })) {
        println!("updating done!");
    }

    Ok(())
}
