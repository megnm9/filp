use std::fs;
use std::io;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::Path;
use std::path::PathBuf;

/// Represents the permissions of a file or directory.
pub struct Permissions {
    path: PathBuf,
    readable: bool,
    writable: bool,
    executable: bool,
}

impl Permissions {
    /// Creates a `Permissions` instance from a file or directory path.
    pub fn from_path(path: PathBuf) -> io::Result<Permissions> {
        let (r, w, x) = check_access(&path)?;

        Ok(Permissions {
            path,
            readable: r,
            writable: w,
            executable: x,
        })
    }

    /// Returns the current Unix permission mode of the file or directory.
    pub fn get_mode(&self) -> std::io::Result<u32> {
        let metadata = fs::metadata(&self.path)?;
        Ok(metadata.mode() & 0o777)
    }

    /// Sets the Unix permission mode of the file or directory.
    pub fn set_mode(&mut self, mode: u32) -> std::io::Result<()> {
        let (r, w, x) = mode_to_rwx(mode);
        fs::set_permissions(&self.path, fs::Permissions::from_mode(mode))?;

        self.readable = r;
        self.writable = w;
        self.executable = x;

        Ok(())
    }

    /// Returns the path associated with this permission snapshot.
    pub fn get_path(&self) -> PathBuf {
        self.path.clone()
    }
    /// Returns whether the owner read bit is set.
    pub fn is_owner_readable(&self) -> bool {
        self.readable
    }
    /// Returns whether the owner write bit is set.
    pub fn is_owner_writable(&self) -> bool {
        self.writable
    }
    /// Returns whether the owner execute bit is set.
    pub fn is_owner_executable(&self) -> bool {
        self.executable
    }
    /// Returns the username from the `USER` environment variable.
    pub fn get_username() -> Result<String, std::env::VarError> {
        std::env::var("USER")
    }
}

fn check_access(path: &Path) -> io::Result<(bool, bool, bool)> {
    let mode = fs::metadata(path)?.permissions().mode();

    let user_readable = mode & 0o400 != 0;
    let user_writable = mode & 0o200 != 0;
    let user_executable = mode & 0o100 != 0;

    Ok((user_readable, user_writable, user_executable))
}

fn mode_to_rwx(mode: u32) -> (bool, bool, bool) {
    let user_digit = (mode >> 6) & 0o7;
    (
        user_digit & 0b100 != 0, // read
        user_digit & 0b010 != 0, // write
        user_digit & 0b001 != 0, // execute
    )
}
