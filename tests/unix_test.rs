#![cfg(unix)]

/*
#[cfg(test)]
mod tests {
    use filp::*;
    use std::fs::{File, remove_file};

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
}
*/

#[cfg(test)]
mod tests {
    use filp::*;
    use std::fs::{self, File};
    use std::os::unix::fs::PermissionsExt;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn test_file() -> PathBuf {
        let id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let path = std::env::temp_dir().join(format!("filp_test_{}_{}", std::process::id(), id));

        File::create(&path).unwrap();
        path
    }

    #[test]
    fn test_owner_rwx() {
        let path = test_file();

        fs::set_permissions(&path, fs::Permissions::from_mode(0o700)).unwrap();

        let permissions = Permissions::from_path(path.clone()).unwrap();

        assert!(permissions.is_owner_readable());
        assert!(permissions.is_owner_writable());
        assert!(permissions.is_owner_executable());

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_owner_read_only() {
        let path = test_file();

        fs::set_permissions(&path, fs::Permissions::from_mode(0o400)).unwrap();

        let permissions = Permissions::from_path(path.clone()).unwrap();

        assert!(permissions.is_owner_readable());
        assert!(!permissions.is_owner_writable());
        assert!(!permissions.is_owner_executable());

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_owner_write_only() {
        let path = test_file();

        fs::set_permissions(&path, fs::Permissions::from_mode(0o200)).unwrap();

        let permissions = Permissions::from_path(path.clone()).unwrap();

        assert!(!permissions.is_owner_readable());
        assert!(permissions.is_owner_writable());
        assert!(!permissions.is_owner_executable());

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_owner_execute_only() {
        let path = test_file();

        fs::set_permissions(&path, fs::Permissions::from_mode(0o100)).unwrap();

        let permissions = Permissions::from_path(path.clone()).unwrap();

        assert!(!permissions.is_owner_readable());
        assert!(!permissions.is_owner_writable());
        assert!(permissions.is_owner_executable());

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_owner_no_permissions() {
        let path = test_file();

        fs::set_permissions(&path, fs::Permissions::from_mode(0o000)).unwrap();

        let permissions = Permissions::from_path(path.clone()).unwrap();

        assert!(!permissions.is_owner_readable());
        assert!(!permissions.is_owner_writable());
        assert!(!permissions.is_owner_executable());

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_owner_full_permissions() {
        let path = test_file();

        fs::set_permissions(&path, fs::Permissions::from_mode(0o777)).unwrap();

        let permissions = Permissions::from_path(path.clone()).unwrap();

        assert!(permissions.is_owner_readable());
        assert!(permissions.is_owner_writable());
        assert!(permissions.is_owner_executable());

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_group_and_other_are_ignored() {
        let path = test_file();

        fs::set_permissions(&path, fs::Permissions::from_mode(0o077)).unwrap();

        let permissions = Permissions::from_path(path.clone()).unwrap();

        assert!(!permissions.is_owner_readable());
        assert!(!permissions.is_owner_writable());
        assert!(!permissions.is_owner_executable());

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_get_mode() {
        let path = test_file();

        fs::set_permissions(&path, fs::Permissions::from_mode(0o754)).unwrap();

        let permissions = Permissions::from_path(path.clone()).unwrap();

        assert_eq!(permissions.get_mode().unwrap(), 0o754);

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_set_mode() {
        let path = test_file();

        let mut permissions = Permissions::from_path(path.clone()).unwrap();

        permissions.set_mode(0o754).unwrap();

        assert_eq!(permissions.get_mode().unwrap(), 0o754);

        assert!(permissions.is_owner_readable());
        assert!(permissions.is_owner_writable());
        assert!(permissions.is_owner_executable());

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_set_mode_updates_self() {
        let path = test_file();

        let mut permissions = Permissions::from_path(path.clone()).unwrap();

        permissions.set_mode(0o700).unwrap();

        assert!(permissions.is_owner_readable());
        assert!(permissions.is_owner_writable());
        assert!(permissions.is_owner_executable());

        permissions.set_mode(0o400).unwrap();

        assert!(permissions.is_owner_readable());
        assert!(!permissions.is_owner_writable());
        assert!(!permissions.is_owner_executable());

        permissions.set_mode(0o200).unwrap();

        assert!(!permissions.is_owner_readable());
        assert!(permissions.is_owner_writable());
        assert!(!permissions.is_owner_executable());

        permissions.set_mode(0o100).unwrap();

        assert!(!permissions.is_owner_readable());
        assert!(!permissions.is_owner_writable());
        assert!(permissions.is_owner_executable());

        permissions.set_mode(0o000).unwrap();

        assert!(!permissions.is_owner_readable());
        assert!(!permissions.is_owner_writable());
        assert!(!permissions.is_owner_executable());

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_get_path() {
        let path = test_file();

        let permissions = Permissions::from_path(path.clone()).unwrap();

        assert_eq!(permissions.get_path(), path);

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_invalid_path() {
        let path = PathBuf::from("/this/path/does/not/exist");

        let result = Permissions::from_path(path);

        assert!(result.is_err());
    }

    #[test]
    fn test_get_username() {
        let username = Permissions::get_username();

        assert!(username.is_ok());
        assert!(!username.unwrap().is_empty());
    }
}
