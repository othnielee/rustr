use anyhow::{Context, Result};
use colored::*;
use home::home_dir;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::constants::*;

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
    println!("          Arguments to pass to the target application\n");

    println!("{}", "Options:".bold().underline());
    println!("      {}", "--build".bold());
    println!("          Build the project");
    println!("      {}", "--release".bold());
    println!("          Build in release mode");
    println!("      {} [<DESTINATION>]", "--release-bin".bold());
    println!("          Build in release mode and copy to ~/bin (or specified path)");
    println!("      {} <PROJECT>", "--project".bold());
    println!("          Explicitly specify the target project");
    println!("      --");
    println!("          Stop option parsing and pass the rest verbatim to the target application");
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
            "Cannot run {} on itself - this would cause infinite recursion",
            app_name
        ))?;
    }
    Ok(())
}

pub fn find_project_dir(project_name: &str) -> Result<PathBuf> {
    // Check if we're in a project directory
    if Path::new(CARGO_TOML).exists() {
        // Try to read its package name - if that fails, fall back to home search
        if let Ok(current_name) = get_binary_name(&PathBuf::from(".")) {
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

pub fn get_binary_name(project_dir: &Path) -> Result<String> {
    let cargo_toml = project_dir.join(CARGO_TOML);
    let contents = fs::read_to_string(cargo_toml)?;

    for line in contents.lines() {
        if line.trim().starts_with(NAME_KEY) {
            let name = line
                .split('=')
                .nth(1)
                .context(format!("Invalid {} format", CARGO_TOML))?
                .trim()
                .trim_matches(|c| c == '"' || c == ' ');
            return Ok(name.to_string());
        }
    }

    bail(&format!("Could not find project name in {}", CARGO_TOML))?;
    unreachable!()
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
