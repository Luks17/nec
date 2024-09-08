use std::fs;

use tempfile::tempfile;
use zip::ZipArchive;

pub fn download_and_install() {
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
