pub mod error;
mod modules;
mod project;

use anyhow::Result;
use project::Project;
use std::path::Path;

pub fn init() -> Result<()> {
    let project = Project::new("output")?;

    project.create_structure()?;

    if Path::new(".nec_modules").exists() {
        println!("Modules already downloaded! Skipping...");
    } else {
        println!("Installing modules...");
        modules::download_and_install();
    }

    Ok(())
}