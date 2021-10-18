use anyhow::Result;

use diplo::{info, load_config::create_deps, term::print_inner, CONFIG, DOTDIPLO};
use serde_json::json;
use std::{fs::write, process::Command};

pub fn exec() -> Result<()> {
    if let Some(dependencies) = &CONFIG.dependencies {
        create_deps(dependencies);
        if let Some(import_map) = CONFIG.import_map {
            if import_map {
                let imports = json!({ "imports": dependencies });
                write(
                    format!("{}/import_map.json", &*DOTDIPLO),
                    serde_json::to_string(&imports).unwrap(),
                )
                .unwrap();
            }
        }
    }
    let mut out = Command::new("deno")
        .args(vec![
            "cache",
            ".diplo/deps.ts",
            "--quiet",
            // "--lock=.diplo/deno-lock.json",
            // "--lock-write",
            // ".diplo/deno-lock.json",
        ])
        .spawn()
        .unwrap();
    if let Err(error) = out.wait() {
        println!("{}", error);
    }
    info!("Successfully cached the dependencies");

    Ok(())
}
