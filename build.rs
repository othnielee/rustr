use chrono::{DateTime, Timelike, Utc};

fn main() {
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=build.rs");

    println!("cargo:rustc-env=APP_NAME={}", env!("CARGO_PKG_NAME"));
    println!("cargo:rustc-env=APP_VERSION={}", env!("CARGO_PKG_VERSION"));

    let timestamp: DateTime<Utc>;

    if cfg!(not(debug_assertions)) {
        let current_time = Utc::now();
        let build_minute = (current_time.minute() / 15) * 15;
        timestamp = current_time.with_minute(build_minute).unwrap();
    } else {
        timestamp = Utc::now();
    }

    let app_build = timestamp.format("%Y%m%d.%H%M").to_string();
    println!("cargo:rustc-env=APP_BUILD={}", app_build);
}
