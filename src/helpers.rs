use anyhow::{Context, Result};
use colored::*;
use home::home_dir;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::constants::*;

#[derive(Default)]
struct BinTarget {
    name: Option<String>,
    path: Option<String>,
}

pub fn print_banner() {
    let app_name = env!("APP_NAME");
    let app_version = env!("APP_VERSION");
    let app_build = env!("APP_BUILD");

    println!("{} v{} (build {})", app_name, app_version, app_build);
}

pub fn print_help() {
    print_banner();

    println!("\nRust/Cargo Task Runner\n");

    println!(
        "{} {} [OPTIONS] [PROJECT_NAME] [ARGS...]\n",
        "Usage:".bold().underline(),
        "rustr".bold()
    );

    println!("{}", "Arguments:".bold().underline());
    println!("  [PROJECT_NAME]");
    println!("          Project name");
    println!("  [ARGS]...");
    println!("          Arguments to pass to the target project\n");

    println!("{}", "Options:".bold().underline());
    println!("      {}", "--test".bold());
    println!("          Run tests for the project");
    println!("      {}", "--build".bold());
    println!("          Build the project");
    println!("      {}", "--release".bold());
    println!("          Build in release mode");
    println!("      {} [<DESTINATION>]", "--release-bin".bold());
    println!("          Build in release mode and copy to ~/bin (or specified path)");
    println!("      {} <PROJECT>", "--project".bold());
    println!("          Explicitly specify the target project");
    println!("      --");
    println!("          Stop option parsing and pass remaining arguments to the target project");
    println!("  -h, --help");
    println!("          Print help");
    println!("  -V, --version");
    println!("          Print version");
}

fn bail(message: &str) -> Result<()> {
    print_banner();
    anyhow::bail!("{}", message);
}

fn is_self_project() -> Result<bool> {
    let app_name = env!("APP_NAME");
    if Path::new(CARGO_TOML).exists() {
        let contents = fs::read_to_string(CARGO_TOML)?;
        for line in contents.lines() {
            if line.trim().starts_with(NAME_KEY) {
                let name = line
                    .split('=')
                    .nth(1)
                    .context(format!("Invalid {} format", CARGO_TOML))?
                    .trim()
                    .trim_matches(|c| c == '"' || c == ' ');
                return Ok(name == app_name);
            }
        }
    }
    Ok(false)
}

pub fn check_self_run(explicit: Option<&str>, positional: Option<&str>) -> Result<()> {
    let app_name = env!("APP_NAME");
    if explicit == Some(app_name)
        || positional == Some(app_name)
        || (explicit.is_none() && positional.is_none() && is_self_project()?)
    {
        bail(&format!(
            "Running {} on itself is not supported.\n\
            Use `cargo build`, `cargo test`, or similar directly inside the repository.",
            app_name
        ))?;
    }
    Ok(())
}

pub fn find_project_dir(project_name: &str) -> Result<PathBuf> {
    // Check if we're in a project directory
    if Path::new(CARGO_TOML).exists() {
        // Try to read its package name - if that fails, fall back to home search
        if let Ok(current_name) = get_package_name(&PathBuf::from(".")) {
            if current_name == project_name {
                return Ok(PathBuf::from("."));
            }
        }
    }

    // Use the canonical project location
    let home = home_dir().context("Could not find home directory")?;
    let project_path = home.join(RUST_PROJECTS_DIR).join(project_name);

    if !project_path.exists() {
        bail(&format!(
            "Project directory not found: {}",
            project_path.display()
        ))?;
    }

    Ok(project_path)
}

pub fn get_package_name(project_dir: &Path) -> Result<String> {
    let cargo_toml = project_dir.join(CARGO_TOML);
    let contents = fs::read_to_string(cargo_toml)?;
    let mut in_package_section = false;

    for line in contents.lines() {
        let trimmed = line.trim();
        if trimmed == "[package]" {
            in_package_section = true;
            continue;
        }
        if trimmed.starts_with('[') {
            in_package_section = false;
            continue;
        }

        if in_package_section {
            if let Some(name) = parse_toml_string_value(trimmed, "name") {
                return Ok(name);
            }
        }
    }

    bail(&format!("Could not find project name in {}", CARGO_TOML))?;
    unreachable!()
}

pub fn get_binary_name(project_dir: &Path) -> Result<String> {
    let package_name = get_package_name(project_dir)?;
    let cargo_toml = project_dir.join(CARGO_TOML);
    let contents = fs::read_to_string(cargo_toml)?;

    let mut in_package_section = false;
    let mut in_bin_section = false;
    let mut default_run = None;
    let mut autobins_enabled = true;
    let mut explicit_bins: Vec<BinTarget> = Vec::new();

    for line in contents.lines() {
        let trimmed = line.trim();
        if trimmed == "[package]" {
            in_package_section = true;
            in_bin_section = false;
            continue;
        }
        if trimmed == "[[bin]]" {
            in_package_section = false;
            in_bin_section = true;
            explicit_bins.push(BinTarget::default());
            continue;
        }
        if trimmed.starts_with('[') {
            in_package_section = false;
            in_bin_section = false;
            continue;
        }

        if in_package_section {
            if default_run.is_none() {
                default_run = parse_toml_string_value(trimmed, "default-run");
            }
            if let Some(autobins) = parse_toml_bool_value(trimmed, "autobins") {
                autobins_enabled = autobins;
            }
        }

        if in_bin_section {
            if let Some(bin) = explicit_bins.last_mut() {
                if bin.name.is_none() {
                    bin.name = parse_toml_string_value(trimmed, "name");
                }
                if bin.path.is_none() {
                    bin.path = parse_toml_string_value(trimmed, "path");
                }
            }
        }
    }

    if let Some(default_run) = default_run {
        return Ok(default_run);
    }

    if let Some(name) = get_explicit_main_bin_name(&explicit_bins) {
        return Ok(name);
    }

    if explicit_bins
        .iter()
        .filter_map(|bin| bin.name.as_deref())
        .any(|name| name == package_name)
    {
        return Ok(package_name);
    }

    if autobins_enabled && project_dir.join("src").join("main.rs").exists() {
        return Ok(package_name);
    }

    let mut explicit_bin_names = explicit_bins
        .into_iter()
        .filter_map(|bin| bin.name)
        .collect::<Vec<_>>();

    if explicit_bin_names.len() == 1 {
        return Ok(explicit_bin_names.pop().unwrap());
    }

    if explicit_bin_names.len() > 1 {
        bail(&format!(
            "Multiple binary targets found in {}. Set [package].default-run or define a binary named '{}'.",
            CARGO_TOML, package_name
        ))?;
    }

    // Fall back to the package name if no explicit binary target exists.
    Ok(package_name)
}

fn get_explicit_main_bin_name(bins: &[BinTarget]) -> Option<String> {
    let mut names = bins
        .iter()
        .filter_map(|bin| {
            let path = bin.path.as_deref()?;
            if is_main_source_path(path) {
                return bin.name.clone();
            }
            None
        })
        .collect::<Vec<_>>();

    if names.len() == 1 {
        return names.pop();
    }
    None
}

fn is_main_source_path(path: &str) -> bool {
    let normalized = path.replace('\\', "/");
    let normalized = normalized.trim_start_matches("./");
    normalized == "src/main.rs"
}

fn parse_toml_string_value(line: &str, key: &str) -> Option<String> {
    let (raw_key, raw_value) = line.split_once('=')?;
    if raw_key.trim() != key {
        return None;
    }

    let value = raw_value.trim_start();
    if !value.starts_with('"') {
        return None;
    }

    let mut parsed = String::new();
    let mut escaped = false;

    for ch in value[1..].chars() {
        if escaped {
            parsed.push(ch);
            escaped = false;
            continue;
        }

        match ch {
            '\\' => escaped = true,
            '"' => return Some(parsed),
            _ => parsed.push(ch),
        }
    }

    None
}

fn parse_toml_bool_value(line: &str, key: &str) -> Option<bool> {
    let (raw_key, raw_value) = line.split_once('=')?;
    if raw_key.trim() != key {
        return None;
    }

    match raw_value.split('#').next()?.trim() {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}

pub fn run_cargo_command(project_dir: &Path, args: &[&str]) -> Result<()> {
    let status = Command::new(CARGO_COMMAND)
        .current_dir(project_dir)
        .args(args)
        .status()?;

    if !status.success() {
        bail(&format!("Command '{}' failed", CARGO_COMMAND))?;
    }

    Ok(())
}

pub fn copy_bin(project_dir: &Path, binary_name: &str, dest_dir: Option<&str>) -> Result<()> {
    let dest_path = if let Some(dir) = dest_dir {
        PathBuf::from(dir)
    } else {
        home_dir()
            .context("Could not find home directory")?
            .join(BIN_DIR)
    };

    if !dest_path.exists() {
        fs::create_dir_all(&dest_path)?;
    }

    let source = project_dir
        .join(TARGET_DIR)
        .join(RELEASE_DIR)
        .join(format!("{}{}", binary_name, BINARY_EXTENSION));

    if !source.exists() {
        bail(&format!(
            "Binary not found: {}. Make sure the build completed successfully.",
            source.display()
        ))?;
    }

    let dest = dest_path.join(format!("{}{}", binary_name, BINARY_EXTENSION));
    fs::copy(&source, &dest)?;
    println!("Copied {} to {}", binary_name, dest_path.display());

    Ok(())
}
