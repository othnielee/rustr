use rustr::constants::{CARGO_TOML, RUST_PROJECTS_DIR};
use rustr::helpers::{get_binary_name, projects_dir_from};
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

struct TempProject {
    path: PathBuf,
}

impl TempProject {
    fn new(cargo_toml: &str) -> Self {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!("rustr-helper-tests-{unique}"));
        fs::create_dir_all(&path).unwrap();
        fs::write(path.join(CARGO_TOML), cargo_toml).unwrap();
        Self { path }
    }

    fn write_file(&self, relative_path: &str, contents: &str) {
        let path = self.path.join(relative_path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, contents).unwrap();
    }
}

impl Drop for TempProject {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

#[test]
fn binary_name_prefers_package_name_over_first_explicit_bin() {
    let project = TempProject::new(
        r#"
[package]
name = "app"
version = "0.1.0"

[[bin]]
name = "helper"
path = "src/bin/helper.rs"

[[bin]]
name = "app"
path = "src/main.rs"
"#,
    );

    let binary = get_binary_name(&project.path).unwrap();
    assert_eq!(binary, "app");
}

#[test]
fn binary_name_uses_default_run_when_present() {
    let project = TempProject::new(
        r#"
[package]
name = "app"
version = "0.1.0"
default-run = "worker"

[[bin]]
name = "helper"
path = "src/bin/helper.rs"

[[bin]]
name = "worker"
path = "src/bin/worker.rs"
"#,
    );

    let binary = get_binary_name(&project.path).unwrap();
    assert_eq!(binary, "worker");
}

#[test]
fn binary_name_uses_explicit_src_main_bin_name_when_renamed() {
    let project = TempProject::new(
        r#"
[package]
name = "alpha-cli"
version = "1.0.0"

[[bin]]
name = "alpha"
path = "src/main.rs"
"#,
    );
    project.write_file("src/main.rs", "fn main() {}");

    let binary = get_binary_name(&project.path).unwrap();
    assert_eq!(binary, "alpha");
}

#[test]
fn binary_name_uses_package_name_when_src_main_exists() {
    let project = TempProject::new(
        r#"
[package]
name = "app"
version = "0.1.0"

[[bin]]
name = "helper-a"
path = "src/bin/helper-a.rs"

[[bin]]
name = "helper-b"
path = "src/bin/helper-b.rs"
"#,
    );
    project.write_file("src/main.rs", "fn main() {}");

    let binary = get_binary_name(&project.path).unwrap();
    assert_eq!(binary, "app");
}

#[test]
fn binary_name_uses_single_explicit_bin_when_no_package_bin_exists() {
    let project = TempProject::new(
        r#"
[package]
name = "app"
version = "0.1.0"
autobins = false

[[bin]]
name = "worker"
path = "src/bin/worker.rs"
"#,
    );

    let binary = get_binary_name(&project.path).unwrap();
    assert_eq!(binary, "worker");
}

#[test]
fn binary_name_errors_on_ambiguous_explicit_bins() {
    let project = TempProject::new(
        r#"
[package]
name = "app"
version = "0.1.0"
autobins = false

[[bin]]
name = "worker-a"
path = "src/bin/worker-a.rs"

[[bin]]
name = "worker-b"
path = "src/bin/worker-b.rs"
"#,
    );

    let error = get_binary_name(&project.path).unwrap_err().to_string();
    assert!(error.contains("Multiple binary targets"));
}

#[test]
fn projects_dir_uses_override_when_set() {
    let resolved = projects_dir_from(Some(OsStr::new("/beta/projects")), Path::new("/alpha"));
    assert_eq!(resolved, PathBuf::from("/beta/projects"));
}

#[test]
fn projects_dir_defaults_under_home_when_override_is_unset() {
    let resolved = projects_dir_from(None, Path::new("/alpha"));
    assert_eq!(resolved, Path::new("/alpha").join(RUST_PROJECTS_DIR));
}

#[test]
fn projects_dir_treats_empty_override_as_unset() {
    let resolved = projects_dir_from(Some(OsStr::new("")), Path::new("/alpha"));
    assert_eq!(resolved, Path::new("/alpha").join(RUST_PROJECTS_DIR));
}
