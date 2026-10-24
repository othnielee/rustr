use chrono::{DateTime, Timelike, Utc};

fn main() {
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=build.rs");

    println!("cargo:rustc-env=APP_NAME={}", env!("CARGO_PKG_NAME"));
    println!("cargo:rustc-env=APP_VERSION={}", env!("CARGO_PKG_VERSION"));

    // Bucket down to a 15-min boundary in release builds to reduce build-time churn
    let timestamp: DateTime<Utc> = if cfg!(not(debug_assertions)) {
        let now = Utc::now();
        let rounded_min = (now.minute() / 15) * 15;
        now.with_minute(rounded_min).unwrap()
    } else {
        Utc::now()
    };

    println!(
        "cargo:rustc-env=APP_BUILD={}",
        timestamp.format("%Y%m%d.%H%M")
    );

    println!(
        "cargo:rustc-env=APP_COMMIT={}",
        git_output(&["rev-parse", "--short", "HEAD"]).unwrap_or_else(|| "unknown".to_string())
    );
    watch_git_head();
}

/// Registers the git paths that change when HEAD moves as rerun triggers,
/// so APP_COMMIT tracks the checked-out commit across commits that touch no
/// other watched path. The `refs` directory is watched whole because a
/// commit after `git pack-refs` creates the branch's loose ref file anew.
fn watch_git_head() {
    for git_path in ["HEAD", "packed-refs", "refs"] {
        // Cargo reruns the build script on every build for a watched path
        // that does not exist.
        if let Some(path) = git_output(&["rev-parse", "--git-path", git_path])
            && std::path::Path::new(&path).exists()
        {
            println!("cargo:rerun-if-changed={path}");
        }
    }
}

fn git_output(args: &[&str]) -> Option<String> {
    let output = std::process::Command::new("git").args(args).output().ok()?;
    if !output.status.success() {
        return None;
    }
    let text = String::from_utf8(output.stdout).ok()?;
    let text = text.strip_suffix('\n').unwrap_or(&text).to_string();
    (!text.is_empty()).then_some(text)
}
