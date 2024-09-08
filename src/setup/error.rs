use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("No supported package manager is installed!\nRequired: npm or yarn")]
    NoSupportedPkgManager,
    #[error("The package manager path is invalid!")]
    InvalidPkgManagerPath,

    #[error("Could not spawn child process!")]
    FailedToRunChildProcess,

    #[error("There is already a generated project!")]
    OutputProjectAlreadyGenerated,

    #[error("Failed to write directory: {0}")]
    CouldNotWriteDir(String),
    #[error("Failed to copy directory: {0}")]
    CouldNotCopyDir(String),

    #[error("Failed to save current directory!")]
    FailedToSaveCurrentDir,
    #[error("Failed to change current directory!")]
    CouldNotChangeCurrentDir,
}
