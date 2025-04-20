// Configuration constants
pub const CARGO_COMMAND: &str = "cargo";
pub const CARGO_TOML: &str = "Cargo.toml";
pub const RUST_PROJECTS_DIR: &str = "dev/Rust";
pub const BIN_DIR: &str = "bin";
pub const TARGET_DIR: &str = "target";
pub const RELEASE_DIR: &str = "release";
pub const NAME_KEY: &str = "name =";

// OS-specific binary extension
#[cfg(windows)]
pub const BINARY_EXTENSION: &str = ".exe";
#[cfg(not(windows))]
pub const BINARY_EXTENSION: &str = "";
