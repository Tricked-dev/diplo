use crate::{run_utils::create_deps, CONFIG, DOTDIPLO};
use anyhow::Result;
use colored::Colorize;
use serde_json::json;
use std::{
    fs::{create_dir_all, write},
};

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
    if let Err(e) = create_dir_all(&*DOTDIPLO) {
        println!("Error while creating {}", &*DOTDIPLO.red());
        println!("{}", format!("{:#?}", e).red());
        return Ok(());
    }

    println!("Successfully cached the dependencies");

    Ok(())
}
