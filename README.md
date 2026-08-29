<div align="center">

# filp

**Cross-platform file permission management for Rust(windows in dev)**

`filp` is a small, dependency-free library for reading and writing file
permissions with a clean, human-friendly API built around explicit permission
bit constants.

![Platforms: Unix ✓, Windows ⏳](https://img.shields.io/badge/platform-unix%20%7C%20windows%20(soon)-3da7db)
![License: MIT](https://img.shields.io/badge/license-MIT-green)
![Version](https://img.shields.io/badge/version-0.1.0-blue)

</div>

---

## Table of Contents

- [Installing](#installing)
- [Usage](#usage)
  - [Reading permissions](#reading-permissions)
  - [Setting permissions](#setting-permissions)
  - [Permission constants](#permission-constants)
  - [Combining permissions](#combining-permissions)
- [Platform support](#platform-support)
- [License](#license)

---

## Installing

Add `filp` to your `Cargo.toml`:

```toml
[dependencies]
filp = "0.2.0"
```

---

## Usage

### Reading permissions

Get the current permission mode (`u32`) of a file or directory:

```rust
use filp::Permissions;

fn main() -> std::io::Result<()> {
    
    let perm = Permissions::from_path("my_file.txt");
    
    println!("File mode: {:o}", perm.get_mode()?);
    
    Ok(())
}
```

### Setting permissions

Set the exact permission mode of a file:

```rust
use filp::Permissions;
use filp::types::{FULL, READ_ONLY};

fn main() -> std::io::Result<()> {
    let perm = Permissions::from_path("my_file.txt");

    perm.set_mode(READ_ONLY)?;

    println!("File mode: {:o}", perm.get_mode()?);

    perm.set_mode(FULL)?;

    println!("File mode: {:o}", perm.get_mode()?);

    Ok(())
}
```

### Permission constants

`filp` exposes named constants instead of magic numbers. Each maps to a familiar
`chmod` value:

| Constant  | Octal | Meaning                       |
| --------- | ----- | ----------------------------- |
| `OWNER_READ`    | `0o400` | Read for the owner     |
| `OWNER_WRITE`   | `0o200` | Write for the owner    |
| `OWNER_EXECUTE` | `0o100` | Execute for the owner  |
| `GROUP_READ`    | `0o040` | Read for the group     |
| `GROUP_WRITE`   | `0o020` | Write for the group    |
| `GROUP_EXECUTE` | `0o010` | Execute for the group  |
| `OTHER_READ`    | `0o004` | Read for others        |
| `OTHER_WRITE`   | `0o002` | Write for others       |
| `OTHER_EXECUTE` | `0o001` | Execute for others     |

And a few handy pre-built combinations:

| Constant     | Octal   | Use case                                |
| ------------ | ------- | --------------------------------------- |
| `READ_ONLY`  | `0o444` | Read-only for owner, group, and others  |
| `READ_WRITE` | `0o666` | Read/write for owner, group, and others |
| `EXECUTABLE` | `0o755` | Executable scripts/binaries             |
| `PRIVATE`    | `0o700` | Private owner-only files                |
| `FULL`       | `0o777` | Everything enabled                      |

### Combining permissions

`filp`'s constants are plain bit flags, so you can combine them freely to build
exactly the permission set you want:

```rust
use filp::Permissions;
use filp::types::{OWNER_READ, OWNER_WRITE, OWNER_EXECUTE, GROUP_READ, OTHER_READ};

fn main() -> std::io::Result<()> {
    // Owner can read/write/execute; group and others can read.
    let mode = OWNER_READ | OWNER_WRITE | OWNER_EXECUTE | GROUP_READ | OTHER_READ;
    //                    ^ 0o100                        ^ 0o040        ^ 0o004
    //                    == 0o744

    let perm = Permissions::from_path("my_file.txt");
    
    perm.set_mode(mode)?;
    Ok(())
}
```

You can also `AND` with a mask to inspect a specific permission:

```rust
use filp::Permissions;
use filp::types::OWNER_READ;

fn main() -> std::io::Result<()> {
    let perm = Permissions::from_path("my_file.txt");
    let mode = perm.get_mode()?;
    let owner_can_read = mode & OWNER_READ != 0;
    println!("owner can read: {owner_can_read}");
    Ok(())
}
```

---

## Platform support

| Platform | Status                    |
| -------- | ------------------------- |
| Unix     | ✅ Fully implemented      |
| Windows  | ⏳ Coming soon            |

On non-Unix platforms the `unix` module is not compiled. Windows support is
being developed and will live behind the `win` module.

---

## License

`filp` is licensed under the [MIT License](LICENSE).
