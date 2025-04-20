mod cli;
mod constants;
mod helpers;

use anyhow::Result;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::cli::*;
use crate::constants::*;
use crate::helpers::{
    check_self_run, copy_bin, find_project_dir, get_binary_name, print_banner, print_help,
    run_cargo_command,
};

fn main() -> Result<()> {
    let args = parse_args()?;

    // Check for self-run
    check_self_run(args.project.as_deref(), args.project_name.as_deref())?;

    // Determine which project to use (--project flag takes precedence)
    let project_name = if let Some(project) = args.project {
        project
    } else if let Some(project_name) = args.project_name {
        project_name
    } else if Path::new(CARGO_TOML).exists() {
        // Get project name from current directory's Cargo.toml
        get_binary_name(&PathBuf::from("."))?
    } else {
        print_help();
        return Ok(());
    };

    // If any of our specific flags are set, do that action and exit

    if args.test {
        print_banner();
        let project_dir = find_project_dir(&project_name)?;
        run_cargo_command(&project_dir, &["test"])?;
        println!("Test complete");
        return Ok(());
    }

    if args.build {
        print_banner();
        let project_dir = find_project_dir(&project_name)?;
        let binary_name = get_binary_name(&project_dir)?;
        println!("Building project: {}", binary_name);
        run_cargo_command(&project_dir, &["build"])?;
        println!("Build complete");
        return Ok(());
    }

    if args.release {
        print_banner();
        let project_dir = find_project_dir(&project_name)?;
        let binary_name = get_binary_name(&project_dir)?;
        println!("Building release version of project: {}", binary_name);
        run_cargo_command(&project_dir, &["build", "--release"])?;
        println!("Release build complete");
        return Ok(());
    }

    if let Some(dest) = args.release_bin {
        print_banner();
        let project_dir = find_project_dir(&project_name)?;
        let binary_name = get_binary_name(&project_dir)?;
        println!("Building release version of project: {}", binary_name);
        run_cargo_command(&project_dir, &["build", "--release"])?;
        println!(
            "Copying {} to {}",
            binary_name,
            dest.as_deref().unwrap_or(BIN_DIR)
        );
        copy_bin(&project_dir, &binary_name, dest.as_deref())?;
        println!("Done");
        return Ok(());
    }

    // If we get here, we're running the target project
    let project_dir = find_project_dir(&project_name)?;
    let binary_name = get_binary_name(&project_dir)?;

    // Build in release mode
    run_cargo_command(&project_dir, &["build", "--release"])?;

    // Run the app with all remaining arguments
    let binary_path = project_dir
        .join(TARGET_DIR)
        .join(RELEASE_DIR)
        .join(format!("{}{}", binary_name, BINARY_EXTENSION));
    let status = Command::new(binary_path)
        .args(&args.project_args)
        .status()?;

    // Pass through the application's exit code
    std::process::exit(status.code().unwrap_or(1));
}
