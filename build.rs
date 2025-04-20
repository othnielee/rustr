use chrono::{DateTime, Timelike, Utc};

fn main() {
    // Re‑run build script when sources or this file change
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=build.rs");

    // Pass package metadata through to the code
    println!("cargo:rustc-env=APP_NAME={}", env!("CARGO_PKG_NAME"));
    println!("cargo:rustc-env=APP_VERSION={}", env!("CARGO_PKG_VERSION"));

    // Timestamp rounded to the nearest 15 min in release builds, full in debug
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
}
