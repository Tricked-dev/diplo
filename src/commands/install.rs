use crate::run_utils::ensure_dependencies;
use anyhow::Result;

pub fn exec() -> Result<()> {
    ensure_dependencies()?;

    println!("Successfully initialized diplo");

    Ok(())
}
