use crate::{load_config::create_deps, CONFIG, DOTDIPLO};
use anyhow::Result;
use serde_json::json;
use std::fs::write;

pub fn exec() -> Result<()> {
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
    println!("Successfully initialized diplo");

    Ok(())
}
