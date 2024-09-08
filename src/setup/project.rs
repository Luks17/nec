use super::error::Error;
use crate::core::utils::copy_dir_all;
use std::path::{Path, PathBuf};
use strum::IntoEnumIterator;
use which::which;

#[derive(strum_macros::Display, strum_macros::EnumIter)]
enum SupportedPkgManagers {
    #[strum(to_string = "npm")]
    Npm,
    #[strum(to_string = "yarn")]
    Yarn,
}

pub struct Project {
    output_path: String,
    pkg_manager: PathBuf,
}

impl Project {
    pub fn new(output_path: &str) -> Result<Self, Error> {
        let pkg_manager = Self::check_pkg_manager()?;

        Ok(Self {
            pkg_manager,
            output_path: output_path.to_string(),
        })
    }

    fn check_pkg_manager() -> Result<PathBuf, Error> {
        for manager in SupportedPkgManagers::iter() {
            if let Ok(manager_executable) = which(manager.to_string()) {
                println!("Found supported pkg manager: {}", manager);
                return Ok(manager_executable);
            }
        }

        Err(Error::NoSupportedPkgManager)
    }

    fn is_output_path_clean(&self) -> Result<(), Error> {
        if Path::new(&self.output_path).exists() {
            return Err(Error::OutputProjectAlreadyGenerated);
        }

        Ok(())
    }

    pub fn create_structure(&self) -> Result<(), Error> {
        self.is_output_path_clean()?;

        let directories = vec![
            (".nec_modules/config", ""),
            (".nec_modules/components", "src/app/(components)"),
            (".nec_modules/database", "src/database"),
            (".nec_modules/lib", "src/lib"),
        ];

        for (source, destination_suffix) in directories {
            let target_dir = source.to_string();
            let destination = format!("{}/{}", self.output_path, destination_suffix);
            copy_dir_all(&target_dir, destination)
                .map_err(|_| Error::CouldNotCopyDir(target_dir))?;
        }

        Ok(())
    }
}
