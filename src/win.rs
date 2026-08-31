use std::fs::OpenOptions;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Describes the current user's effective permissions for a Windows path.
///
/// Windows access-control entries are represented as owner-style read, write,
/// and execute flags for a consistent cross-platform API.
pub struct Permissions {
    path: PathBuf,
    readable: bool,
    writable: bool,
    executable: bool,
}

impl Permissions {
    /// Creates a permission snapshot for `filepath`.
    ///
    /// Read and write access are determined by attempting to open the path;
    /// execute access is inferred from common executable file extensions.
    pub fn from_path(filepath: PathBuf) -> Permissions {
        let (r, w, e) = check_access(&filepath);

        Permissions {
            path: filepath,
            readable: r,
            writable: w,
            executable: e,
        }
    }

    /// Returns a Unix-style permission mode representing these flags.
    ///
    /// The owner bits reflect the stored flags, while group and other are
    /// always reported as read-only.
    pub fn get_mode(&self) -> u32 {
        rwx_to_mode(self.readable, self.writable, self.executable)
    }

    /// Updates this path's access-control entries for the current user.
    ///
    /// The supplied Unix-style owner bits are translated to `icacls` read,
    /// write, and execute permissions.
    pub fn set_mode(&mut self, mode: u32) -> io::Result<()> {
        let (r, w, x) = mode_to_rwx(mode);
        let path = self.path.to_str().unwrap();
        set_access(path, r, w, x)?;

        self.readable = r;
        self.writable = w;
        self.executable = x;

        Ok(())
    }

    /// Returns the path associated with this permission snapshot.
    pub fn get_path(&self) -> PathBuf {
        self.path.clone()
    }
    /// Returns whether the current user can read the path.
    pub fn is_owner_readable(&self) -> bool {
        self.readable
    }
    /// Returns whether the current user can write to the path.
    pub fn is_owner_writable(&self) -> bool {
        self.writable
    }
    /// Returns whether the current user can execute the path.
    pub fn is_owner_executable(&self) -> bool {
        self.executable
    }
    /// Returns the current Windows account name.
    pub fn get_username() -> io::Result<String> {
        current_user()
    }
}

fn rwx_to_mode(read: bool, write: bool, execute: bool) -> u32 {
    let user_digit = (read as u32) << 2 | (write as u32) << 1 | (execute as u32);
    (user_digit << 6) | 0o044 // group and others fixed at r--
}

fn mode_to_rwx(mode: u32) -> (bool, bool, bool) {
    let user_digit = (mode >> 6) & 0o7;
    (
        user_digit & 0b100 != 0, // read
        user_digit & 0b010 != 0, // write
        user_digit & 0b001 != 0, // execute
    )
}

fn check_access(path: &Path) -> io::Result<(bool, bool, bool)> {
    let readable = OpenOptions::new().read(true).open(path).is_ok();

    let writable = OpenOptions::new().write(true).open(path).is_ok();

    // Windows does not have Unix-style execute permission bits.
    // For regular files, determine executability by extension.
    let executable = path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| {
            matches!(
                ext.to_lowercase().as_str(),
                "exe" | "bat" | "cmd" | "com" | "msi" | "ps1"
            )
        })
        .unwrap_or(false);

    Ok((readable, writable, executable))
}

fn set_access(path: &str, readable: bool, writable: bool, executable: bool) -> io::Result<()> {
    let user = current_user()?;

    let mut grant_perms = Vec::new();
    let mut deny_perms = Vec::new();

    if readable {
        grant_perms.push("RD");
    } else {
        deny_perms.push("RD");
    }
    if writable {
        grant_perms.push("WD");
    } else {
        deny_perms.push("WD");
    }
    if executable {
        grant_perms.push("X");
    } else {
        deny_perms.push("X");
    }

    if !grant_perms.is_empty() {
        run_icacls(path, &user, "/grant", &grant_perms)?;
    }
    if !deny_perms.is_empty() {
        run_icacls(path, &user, "/deny", &deny_perms)?;
    }

    Ok(())
}

fn run_icacls(path: &str, user: &str, mode: &str, perms: &[&str]) -> io::Result<()> {
    let perm_str = format!("{}:({})", user, perms.join(","));

    let output = Command::new("icacls")
        .arg(path)
        .arg(mode)
        .arg(&perm_str)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("icacls failed: {}", stderr.trim()),
        ));
    }

    Ok(())
}

fn current_user() -> io::Result<String> {
    let output = Command::new("whoami").output()?;
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}
