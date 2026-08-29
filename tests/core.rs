use filp::Permissions;
use std::fs;
use std::path::PathBuf;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

#[test]
fn test_from_path() {
    let path = PathBuf::from("test_file.txt");
    let permissions = Permissions::from_path(&path);

    assert_eq!(permissions.get_path(), path);
}

#[cfg(unix)]
#[test]
fn test_unix_get_mode() {
    let path = PathBuf::from("test_unix_get_mode.txt");

    fs::write(&path, "test").unwrap();

    fs::set_permissions(&path, fs::Permissions::from_mode(0o644)).unwrap();

    let permissions = Permissions::from_path(&path);

    assert_eq!(permissions.get_mode().unwrap(), 0o644);

    fs::remove_file(&path).unwrap();
}

#[cfg(unix)]
#[test]
fn test_unix_set_mode() {
    let path = PathBuf::from("test_unix_set_mode.txt");

    fs::write(&path, "test").unwrap();

    let permissions = Permissions::from_path(&path);

    permissions.set_mode(0o600).unwrap();

    assert_eq!(permissions.get_mode().unwrap(), 0o600);

    fs::remove_file(&path).unwrap();
}

#[cfg(windows)]
#[test]
fn test_windows_get_mode() {
    let path = PathBuf::from("test_windows_get_mode.txt");

    fs::write(&path, "test").unwrap();

    let permissions = Permissions::from_path(&path);

    // Normal Windows file should be writable.
    assert_eq!(permissions.get_mode().unwrap(), 0o666);

    fs::remove_file(&path).unwrap();
}

#[cfg(windows)]
#[test]
fn test_windows_set_readonly() {
    let path = PathBuf::from("test_windows_readonly.txt");

    fs::write(&path, "test").unwrap();

    let permissions = Permissions::from_path(&path);

    // Remove write bits -> Windows READONLY.
    permissions.set_mode(0o444).unwrap();

    assert_eq!(permissions.get_mode().unwrap(), 0o444);

    // Add write bits -> remove Windows READONLY.
    permissions.set_mode(0o666).unwrap();

    assert_eq!(permissions.get_mode().unwrap(), 0o666);

    fs::remove_file(&path).unwrap();
}

#[cfg(windows)]
#[test]
fn test_windows_directory_mode() {
    let path = PathBuf::from("test_windows_directory");

    fs::create_dir(&path).unwrap();

    let permissions = Permissions::from_path(&path);

    // Writable directory = 0o777 in your emulation.
    assert_eq!(permissions.get_mode().unwrap(), 0o777);

    fs::remove_dir(&path).unwrap();
}

/* #[cfg(test)]
mod tests {
    use filp::*;
    use std::fs;
    use std::fs::{File, remove_file};
    use std::io::Write;

    #[test]
    fn test_get_mode() {
        let path = "test_get_mode.txt";

        File::create(path).unwrap();

        let perm = Permissions::from_path(path);

        perm.set_mode(0o644).unwrap();

        assert_eq!(perm.get_mode().unwrap(), 0o644);

        perm.set_mode(FULL).unwrap();

        assert_eq!(perm.get_mode().unwrap(), FULL);

        remove_file(path).unwrap();
    }

    #[test]
    fn round_trips_mode_display() {
        let mut file = tempfile();
        write!(file, "hi").unwrap();
        let path = file.path().to_path_buf();
        drop(file);

        let p = Permissions::from_path(path.clone());
        let mode = p.get_mode().unwrap();
        println!("mode = {}", Permissions::mode_to_string(mode));

        p.set_mode(0o444).unwrap();
        let mode2 = p.get_mode().unwrap();
        #[cfg(unix)]
        assert_eq!(mode2 & 0o777, 0o444);
        #[cfg(windows)]
        assert_eq!(mode2, 0o444);

        fs::remove_file(&path).ok();
    }

    fn tempfile() -> std::fs::File {
        let dir = std::env::temp_dir();
        let path = dir.join(format!("filp_test_{}.txt", std::process::id()));
        std::fs::File::create(path).unwrap()
    }
}
*/
