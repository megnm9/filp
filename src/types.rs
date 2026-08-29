/// Owner read permission.
pub const OWNER_READ: u32 = 0o400;

/// Owner write permission.
pub const OWNER_WRITE: u32 = 0o200;

/// Owner execute permission.
pub const OWNER_EXECUTE: u32 = 0o100;

/// Group read permission.
pub const GROUP_READ: u32 = 0o040;

/// Group write permission.
pub const GROUP_WRITE: u32 = 0o020;

/// Group execute permission.
pub const GROUP_EXECUTE: u32 = 0o010;

/// Other users read permission.
pub const OTHER_READ: u32 = 0o004;

/// Other users write permission.
pub const OTHER_WRITE: u32 = 0o002;

/// Other users execute permission.
pub const OTHER_EXECUTE: u32 = 0o001;

/// Read permission for owner, group, and other users.
pub const READ_ONLY: u32 = 0o444;

/// Read and write permission for owner, group, and other users.
pub const READ_WRITE: u32 = 0o666;

/// Read, write, and execute permission for the owner only.
pub const PRIVATE: u32 = 0o700;

/// Read, write, and execute permission for owner, group, and other users.
pub const FULL: u32 = 0o777;

// Common permission combinations
pub const EXECUTABLE: u32 = 0o755;
