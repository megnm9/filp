#[cfg(test)]
mod tests {
    use filp::types::*;
    use filp::unix::*;
    use std::fs::{self, File};
    use std::os::unix::fs::PermissionsExt;

    #[test]
    fn test_get_mode() {
        let path = "test_get_mode.txt";

        File::create(path).unwrap();

        fs::set_permissions(path, fs::Permissions::from_mode(0o644)).unwrap();

        let mode = get_mode(path).unwrap();

        assert_eq!(mode & 0o777, 0o644);

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_set_mode() {
        let path = "test_set_mode.txt";

        File::create(path).unwrap();

        set_mode(path, OWNER_READ | OWNER_EXECUTE | OTHER_READ).unwrap();

        let mode = get_mode(path).unwrap();

        assert_eq!(mode & 0o777, OWNER_READ | OWNER_EXECUTE | OTHER_READ);

        fs::remove_file(path).unwrap();
    }
}
