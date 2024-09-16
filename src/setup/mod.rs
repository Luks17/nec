mod error;
mod modules;
pub mod project;

use anyhow::Result;
use project::Project;
use std::path::Path;

pub fn init_project() -> Result<Project> {
    println!("Initializing project...");
    let project = Project::new("output")?;

    if Path::new(".nec_modules").exists() {
        println!("Core modules already downloaded! Skipping...");
    } else {
        println!("Installing core modules...");
        modules::download_and_install();
    }

    println!("Copying core modules to project...");
    project.create_structure()?;
    println!(
        "Running {} install...",
        project.pkg_manager.to_string_lossy()
    );
    // project.install_deps()?;

    Ok(project)
}
