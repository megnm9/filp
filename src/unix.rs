use std::fs;
use std::io;
use std::path::Path;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

/// Returns the complete Unix file mode for the file at `path`.
///
/// The returned value contains both the file type bits and permission bits.
/// For example, a regular file with `644` permissions may return `0o100644`.
///
/// # Errors
///
/// Returns an [`std::io::Error`] if the file metadata cannot be read.
///
/// # Example
///
/// ```no_run
/// use filp::unix::get_mode;
///
/// let mode = get_mode("file.txt")?;
/// println!("{mode:o}");
/// # Ok::<(), std::io::Error>(())
/// ```
#[cfg(unix)]
pub fn get_mode(path: impl AsRef<Path>) -> io::Result<u32> {
    Ok(fs::metadata(path)?.permissions().mode())
}

/// Sets the Unix permission mode of the file at `path`.
///
/// The provided `mode` replaces the file's existing permission bits.
///
/// # Errors
///
/// Returns an [`std::io::Error`] if the file permissions cannot be changed.
///
/// # Example
///
/// ```no_run
/// use filp::unix::set_mode;
///
/// set_mode("file.txt", 0o644)?;
/// # Ok::<(), std::io::Error>(())
/// ```
#[cfg(unix)]
pub fn set_mode(path: impl AsRef<Path>, mode: u32) -> io::Result<()> {
    fs::set_permissions(path, fs::Permissions::from_mode(mode))
}
