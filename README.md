<div align="center">

# filp

**Cross-platform file-permission management for Rust**

`filp` is a small, dependency-free library for inspecting and updating file
permissions on Unix and Windows.

![Platforms: Unix and Windows](https://img.shields.io/badge/platform-unix%20%7C%20windows-3da7db)
![License: MIT](https://img.shields.io/badge/license-MIT-green)
![Version](https://img.shields.io/badge/version-0.3.0-blue)

</div>

---

## Installing

Add `filp` to your `Cargo.toml`:

```toml
[dependencies]
filp = "0.3.0"
```

---

## Usage

`Permissions` stores a snapshot of a path's owner read, write, and execute
permissions. Construct it with a `PathBuf`:

```rust
use filp::Permissions;
use std::path::PathBuf;

let path = PathBuf::from("my_file.txt");
let permissions = Permissions::from_path(path)?;

assert!(permissions.is_owner_readable());
println!("Path: {}", permissions.get_path().display());
# Ok::<(), std::io::Error>(())
```

On Unix, `from_path` returns `io::Result<Permissions>` and `get_mode` returns
`io::Result<u32>`. Windows returns `Permissions` and `u32` directly; see the
[Windows notes](#windows-notes) for its current limitations.

### Reading and setting modes on Unix

Use familiar Unix-style octal modes to read or update a path:

```rust
use filp::Permissions;
use std::path::PathBuf;

let path = PathBuf::from("my_file.txt");
let mut permissions = Permissions::from_path(path)?;

permissions.set_mode(0o644)?;
println!("File mode: {:o}", permissions.get_mode()?);
# Ok::<(), std::io::Error>(())
```

After `set_mode`, the owner-permission query methods reflect the new mode:

```rust
# use filp::Permissions;
# use std::path::PathBuf;
# let mut permissions = Permissions::from_path(PathBuf::from("my_file.txt"))?;
permissions.set_mode(0o700)?;
assert!(permissions.is_owner_readable());
assert!(permissions.is_owner_writable());
assert!(permissions.is_owner_executable());
# Ok::<(), std::io::Error>(())
```

### Permission constants

Named bit flags are re-exported from the crate root and can be combined to
make a mode:

```rust
use filp::{GROUP_READ, OTHER_READ, OWNER_EXECUTE, OWNER_READ, OWNER_WRITE};

let mode = OWNER_READ | OWNER_WRITE | OWNER_EXECUTE | GROUP_READ | OTHER_READ;
assert_eq!(mode, 0o744);
```

| Constant | Octal | Meaning |
| --- | --- | --- |
| `OWNER_READ`, `OWNER_WRITE`, `OWNER_EXECUTE` | `0o400`, `0o200`, `0o100` | Owner permissions |
| `GROUP_READ`, `GROUP_WRITE`, `GROUP_EXECUTE` | `0o040`, `0o020`, `0o010` | Group permissions |
| `OTHER_READ`, `OTHER_WRITE`, `OTHER_EXECUTE` | `0o004`, `0o002`, `0o001` | Other-user permissions |
| `READ_ONLY` | `0o444` | Read-only for owner, group, and others |
| `READ_WRITE` | `0o666` | Read/write for owner, group, and others |
| `EXECUTABLE` | `0o755` | Conventional executable mode |
| `PRIVATE` | `0o700` | Full access for the owner only |
| `FULL` | `0o777` | Full access for everyone |

---

## Platform support

| Platform | Status | Details |
| --- | --- | --- |
| Unix | ✅ Supported | Reads and writes Unix permission modes. Owner query methods inspect owner bits. |
| Windows | ✅ Supported with limitations | Uses `icacls` to manage the current user's ACL entries. |

### Windows notes

Windows support currently handles **only the current user's (owner-style)
permissions**. Group and other permission bits in a Unix-style mode are not
managed as Windows ACL entries.

On Windows, `set_mode` translates the owner read, write, and execute bits to
`icacls` permissions for the current user. `get_mode` represents those stored
flags as owner bits and reports group and other as read-only (`0o044`).
Executability detected when opening a path is based on common executable file
extensions such as `.exe`, `.bat`, `.cmd`, `.com`, `.msi`, and `.ps1`.

---

## License

`filp` is licensed under the [MIT License](LICENSE).
