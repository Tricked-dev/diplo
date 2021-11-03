use crate::{load_config::update_config_toml, update_deno::update_deps, CONFIG, DIPLO_CONFIG};
use anyhow::Result;
use serde_json::json;
use std::fs::read_to_string;
use toml_edit::{value, Document};

pub async fn exec() -> Result<()> {
    let newdeps = update_deps(CONFIG.dependencies.as_ref().unwrap()).await;

    //Cant error cause it would default to json
    let data = read_to_string(&*DIPLO_CONFIG)?;
    let mut document = data.parse::<Document>()?;
    for (name, val) in newdeps.iter() {
        if let Some(exports) = &val.exports {
            document["dependencies"][name] = "{}".parse::<toml_edit::Item>().unwrap();
            document["dependencies"][name]["url"] = value(&val.url);
            document["dependencies"][name]["exports"] = value(exports);
        } else {
            document["dependencies"][name] = value(&val.url);
        }
    }
    update_config_toml(document);

    Ok(())
}
