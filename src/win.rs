use std::path::PathBuf;

pub struct Permissions {
    path: PathBuf,
}

impl Permissions {
    pub fn from_path(fpath: impl Into<PathBuf>) -> Permissions {
        let path = fpath.into();
        Permissions { path }
    }

    pub fn get_path(&self) -> &std::path::Path {
        &self.path
    }

    /// Returns a Unix-style mode (e.g. 0o644, 0o755).
    ///
    /// On Unix this is the real mode bits from the filesystem.
    /// On Windows there is no such thing, so it's emulated from
    /// FILE_ATTRIBUTE_READONLY + whether the path is a directory:
    ///   - read-only file      -> 0o444
    ///   - writable file       -> 0o666
    ///   - directories add     -> 0o111 (traverse bit)
    /// This mirrors the convention CPython uses for os.stat() on Windows.
    pub fn get_mode(&self) -> std::io::Result<u32> {
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            let metadata = std::fs::metadata(&self.path)?;
            Ok(metadata.mode() & 0o7777)
        }

        #[cfg(windows)]
        {
            use std::os::windows::fs::MetadataExt;
            let metadata = std::fs::metadata(&self.path)?;
            Ok(Self::attrs_to_mode(
                metadata.file_attributes(),
                metadata.is_dir(),
            ))
        }
    }

    /// Sets permissions from a Unix-style mode.
    ///
    /// On Unix this sets the real mode bits via chmod.
    /// On Windows only the read-only attribute can be represented:
    ///   - no write bits set (mode & 0o222 == 0) -> mark read-only
    ///   - any write bit set                      -> clear read-only
    /// Read/execute bits are silently ignored on Windows since there's
    /// no equivalent flag for them at this API level.
    pub fn set_mode(&self, mode: u32) -> std::io::Result<()> {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let perms = std::fs::Permissions::from_mode(mode);
            std::fs::set_permissions(&self.path, perms)
        }

        #[cfg(windows)]
        {
            let metadata = std::fs::metadata(&self.path)?;
            let mut perms = metadata.permissions();
            let readonly = mode & 0o222 == 0;
            perms.set_readonly(readonly);
            std::fs::set_permissions(&self.path, perms)
        }
    }

    /// Formats a mode the way you'd see it printed in shell tools, e.g. "0o644".
    pub fn mode_to_string(mode: u32) -> String {
        format!("0o{:o}", mode & 0o7777)
    }

    #[cfg(windows)]
    fn attrs_to_mode(attrs: u32, is_dir: bool) -> u32 {
        const FILE_ATTRIBUTE_READONLY: u32 = 0x1;
        let mut mode = if attrs & FILE_ATTRIBUTE_READONLY != 0 {
            0o444
        } else {
            0o666
        };
        if is_dir {
            mode |= 0o111;
        }
        mode
    }
}
