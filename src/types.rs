// Owner
pub const OWNER_READ: u32 = 0o400;
pub const OWNER_WRITE: u32 = 0o200;
pub const OWNER_EXECUTE: u32 = 0o100;

// Group
pub const GROUP_READ: u32 = 0o040;
pub const GROUP_WRITE: u32 = 0o020;
pub const GROUP_EXECUTE: u32 = 0o010;

// Others
pub const OTHER_READ: u32 = 0o004;
pub const OTHER_WRITE: u32 = 0o002;
pub const OTHER_EXECUTE: u32 = 0o001;

// Common permission combinations
pub const READ_ONLY: u32 = 0o444;
pub const READ_WRITE: u32 = 0o666;
pub const EXECUTABLE: u32 = 0o755;
pub const PRIVATE: u32 = 0o700;
pub const FULL: u32 = 0o777;
