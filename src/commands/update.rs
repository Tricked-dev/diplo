use anyhow::Result;

use diplo::{
    info, load_config::update_config, term::print_inner, update_deno::update_deps, CONFIG,
};
use serde_json::json;

pub async fn exec() -> Result<()> {
    let newdeps = update_deps(CONFIG.dependencies.as_ref().unwrap()).await;
    if let true = update_config(json!({ "dependencies": &newdeps })) {
        info!("updating done!");
    }
    Ok(())
}
