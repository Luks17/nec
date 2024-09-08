use std::{fs, path::Path};

use tempfile::tempfile;
use zip::ZipArchive;

pub fn install_modules() {
    if Path::new(".nec_modules").exists() {
        println!("Modules already downloaded! Skipping...");
        return;
    }

    let mut tmp = tempfile().expect("Failed to create tempfile");
    reqwest::blocking::get("https://github.com/Luks17/nec-modules/archive/refs/heads/main.zip")
        .expect("Request failed")
        .copy_to(&mut tmp)
        .expect("Failed copying result to file");

    let mut zip = ZipArchive::new(tmp).expect("Failed unpacking zip file");

    zip.extract(".")
        .expect("Failed extracting zip file to directory");

    fs::rename("nec-modules-main", ".nec_modules").expect("Failed renaming directory");
}
