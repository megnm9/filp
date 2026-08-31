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
