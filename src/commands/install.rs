use crate::{command_prelude::*, run_utils::ensure_dependencies};
use anyhow::Result;

pub fn cli() -> App<'static> {
    App::new("install").about("This creates the .diplo directory with all required files")
}

pub fn exec() -> Result<()> {
    ensure_dependencies()?;

    println!("Successfully initialized diplo");

    Ok(())
}
