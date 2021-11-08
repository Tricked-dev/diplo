use crate::{
    command_prelude::*, run_utils::create_deps, utils::run_utils::get_dep_urls, CONFIG, DOTDIPLO,
};
use anyhow::Result;
use colored::Colorize;
use serde_json::json;
use std::fs::{create_dir_all, write};

pub fn cli() -> App<'static> {
    App::new("cache").about("Cache the dependencies")
}

pub fn exec() -> Result<()> {
    if let Some(dependencies) = &CONFIG.dependencies {
        create_deps(dependencies);
        if let Some(import_map) = CONFIG.import_map {
            if import_map {
                let import_map = get_dep_urls();

                let imports = json!({ "imports": import_map });
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
