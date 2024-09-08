use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("No supported package manager is installed!\nRequired: npm or yarn")]
    NoSupportedPkgManager,

    #[error("There is already a generated project!")]
    OutputProjectAlreadyGenerated,

    #[error("Failed to write directory: {0}")]
    CouldNotWriteDir(String),
    #[error("Failed to copy directory: {0}")]
    CouldNotCopyDir(String),
}
