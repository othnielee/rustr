use rustr::constants::CARGO_TOML;
use rustr::helpers::get_binary_name;
use std::fs;
use std::path::PathBuf;
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
