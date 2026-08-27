use std::fs;
use std::io;
use std::path::Path;

use std::os::unix::fs::PermissionsExt;

#[cfg(unix)]
pub fn get_mode(path: impl AsRef<Path>) -> io::Result<u32> {
    Ok(fs::metadata(path)?.permissions().mode())
}

#[cfg(unix)]
pub fn set_mode(path: impl AsRef<Path>, mode: u32) -> io::Result<()> {
    fs::set_permissions(path, fs::Permissions::from_mode(mode))
}
