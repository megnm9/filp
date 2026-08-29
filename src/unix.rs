use std::fs;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::PathBuf;

/// Represents the permissions of a file or directory.
pub struct Permissions {
    path: PathBuf,
}

impl Permissions {
    /// Creates a `Permissions` instance from a file or directory path.
    pub fn from_path(path: impl Into<PathBuf>) -> Permissions {
        Permissions { path: path.into() }
    }

    /// Returns the current Unix permission mode of the file or directory.
    pub fn get_mode(&self) -> std::io::Result<u32> {
        let metadata = fs::metadata(&self.path)?;
        Ok(metadata.mode() & 0o777)
    }

    /// Sets the Unix permission mode of the file or directory.
    pub fn set_mode(&self, mode: u32) -> std::io::Result<()> {
        fs::set_permissions(&self.path, fs::Permissions::from_mode(mode))
    }

    /// Checks if the file is readable by the owner.
    pub fn can_read(&self) -> bool {
        let mode = self.get_mode().unwrap_or(0);
        (mode & 0o400) != 0
    }

    /// Returns the path of the file or directory.
    pub fn get_path(&self) -> PathBuf {
        self.path.clone()
    }
}
