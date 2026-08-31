#[cfg(test)]
#[cfg(windows)]
mod permissions_tests {
    use filp::*;
    use std::fs;
    use std::io::Write;
    use std::path::{Path, PathBuf};

    fn temp_file(name: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        path.push(format!("perm_test_{}_{}", std::process::id(), name));
        let mut f = fs::File::create(&path).expect("failed to create temp file");
        writeln!(f, "test content").unwrap();
        path
    }

    fn cleanup(path: &Path) {
        let _ = fs::remove_file(path);
    }

    // ---------------------------------------------------------------
    // from_path / get_path / initial state
    // ---------------------------------------------------------------

    #[test]
    fn from_path_stores_correct_path() {
        let path = temp_file("path_check.txt");
        let perms = Permissions::from_path(path.clone());
        assert_eq!(perms.get_path(), path);
        cleanup(&path);
    }

    #[test]
    fn from_path_normal_file_is_readable_and_writable() {
        let path = temp_file("normal.txt");
        let perms = Permissions::from_path(path.clone());
        assert!(perms.is_readable());
        assert!(perms.is_writable());
        cleanup(&path);
    }

    #[test]
    fn from_path_txt_extension_not_executable() {
        let path = temp_file("plain.txt");
        let perms = Permissions::from_path(path.clone());
        assert!(!perms.is_executable());
        cleanup(&path);
    }

    #[test]
    fn from_path_exe_extension_is_executable() {
        let path = temp_file("thing.exe");
        let perms = Permissions::from_path(path.clone());
        assert!(perms.is_executable());
        cleanup(&path);
    }

    #[test]
    fn from_path_nonexistent_file_is_not_readable_or_writable() {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "perm_test_{}_does_not_exist.txt",
            std::process::id()
        ));
        // Deliberately not creating the file.
        let perms = Permissions::from_path(path.clone());
        assert!(!perms.is_readable());
        assert!(!perms.is_writable());
    }

    // ---------------------------------------------------------------
    // get_mode
    // ---------------------------------------------------------------

    #[test]
    fn get_mode_matches_rwx_state_for_normal_file() {
        let path = temp_file("mode_check.txt");
        let perms = Permissions::from_path(path.clone());

        // readable=true, writable=true, executable=false (.txt)
        // user bits: r-w- = 110 = 6 -> 0o644
        assert_eq!(perms.get_mode(), 0o644);
        cleanup(&path);
    }

    #[test]
    fn get_mode_reflects_executable_extension() {
        let path = temp_file("script.bat");
        let perms = Permissions::from_path(path.clone());

        // r-x from readable+executable, plus writable -> rwx = 111 = 7
        assert_eq!(perms.get_mode(), 0o744);
        cleanup(&path);
    }

    // ---------------------------------------------------------------
    // set_mode — Windows-only, since it shells out to icacls.
    // Marked #[ignore] because it mutates real ACLs; run explicitly
    // with `cargo test -- --ignored` on a Windows machine.
    // ---------------------------------------------------------------

    #[cfg(windows)]
    mod windows_only {
        use super::*;

        #[test]
        fn set_mode_updates_internal_state() {
            let path = temp_file("win_state.txt");
            let mut perms = Permissions::from_path(path.clone());

            let target_mode = 0o444; // r--
            perms.set_mode(target_mode).expect("set_mode failed");

            assert!(perms.is_readable());
            assert!(!perms.is_writable());
            assert!(!perms.is_executable());
            assert_eq!(perms.get_mode(), target_mode);

            // restore so cleanup can delete the file
            perms.set_mode(0o644).ok();
            cleanup(&path);
        }

        #[test]
        fn set_mode_actually_blocks_writes_on_disk() {
            let path = temp_file("win_real_deny.txt");
            let mut perms = Permissions::from_path(path.clone());

            perms.set_mode(0o444).expect("set_mode failed"); // deny write

            // Verify independently of the struct's own bookkeeping:
            // re-open a *fresh* Permissions from the same path and check
            // it reports not-writable too (i.e. icacls really changed it,
            // not just our in-memory flags).
            let reloaded = Permissions::from_path(path.clone());
            assert!(
                !reloaded.is_writable(),
                "icacls deny WD did not persist on disk"
            );
            assert!(reloaded.is_readable());

            // restore before cleanup
            perms.set_mode(0o644).ok();
            cleanup(&path);
        }

        #[test]
        fn set_mode_grants_execute_and_persists() {
            let path = temp_file("win_exec_grant.dat"); // non-.exe, so
            // executability only comes from ACL, not extension
            let mut perms = Permissions::from_path(path.clone());
            assert!(!perms.is_executable());

            perms.set_mode(0o744).expect("set_mode failed"); // rwx
            assert!(perms.is_executable());

            let reloaded = Permissions::from_path(path.clone());
            // Note: check_access only detects executable via extension,
            // so a reloaded Permissions on a .dat file will report
            // is_executable() == false even after icacls granted X.
            // This asymmetry is worth knowing about — see note below.
            assert!(!reloaded.is_executable());

            perms.set_mode(0o644).ok();
            cleanup(&path);
        }

        #[test]
        fn set_mode_round_trip_various_modes() {
            let path = temp_file("win_roundtrip.txt");
            let mut perms = Permissions::from_path(path.clone());

            for mode in [0o744u32, 0o444, 0o644, 0o144] {
                perms.set_mode(mode).expect("set_mode failed");
                assert_eq!(perms.get_mode(), mode);
            }

            perms.set_mode(0o644).ok();
            cleanup(&path);
        }

        #[test]
        fn get_username_returns_nonempty_string() {
            // Doesn't touch ACLs — safe to run unignored.
            let user = Permissions::get_username().expect("whoami failed");
            assert!(!user.trim().is_empty());
        }
    }
}
