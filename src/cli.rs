use anyhow::{anyhow, Result};

use crate::helpers::{print_banner, print_help};

pub struct CliArgs {
    pub build: bool,
    pub release: bool,
    pub release_bin: Option<Option<String>>,
    pub project: Option<String>,
    pub project_name: Option<String>,
    pub app_args: Vec<String>,
}

impl Default for CliArgs {
    fn default() -> Self {
        Self {
            build: false,
            release: false,
            release_bin: None,
            project: None,
            project_name: None,
            app_args: Vec::new(),
        }
    }
}

pub fn parse_args() -> Result<CliArgs> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let mut config = CliArgs::default();
    let mut i = 0;

    // Extract known flags and project specifications
    while i < args.len() {
        let arg = &args[i];

        match arg.as_str() {
            "--help" | "-h" => {
                // Print help and exit
                print_help();
                std::process::exit(0);
            }
            "--version" | "-V" => {
                // Print version and exit
                print_banner();
                std::process::exit(0);
            }
            "--build" => {
                config.build = true;
                i += 1;
            }
            "--release" => {
                config.release = true;
                i += 1;
            }
            arg if arg == "--release-bin" || arg.starts_with("--release-bin=") => {
                if let Some(dest) = arg.strip_prefix("--release-bin=") {
                    // --release-bin=/some/path
                    // --release-bin= (empty → treat as “no value”)
                    config.release_bin = if dest.is_empty() {
                        Some(None)
                    } else {
                        Some(Some(dest.to_owned()))
                    };
                    i += 1; // this flag is self‑contained
                } else if i + 1 < args.len() && !args[i + 1].starts_with("--") {
                    // --release-bin /some/path
                    config.release_bin = Some(Some(args[i + 1].clone()));
                    i += 2;
                } else {
                    // --release-bin (no path supplied)
                    config.release_bin = Some(None);
                    i += 1;
                }
            }
            arg if arg == "--project" || arg.starts_with("--project=") => {
                if let Some(name) = arg.strip_prefix("--project=") {
                    // --project=my_proj
                    if name.is_empty() {
                        return Err(anyhow!("Missing project name after --project"));
                    }
                    config.project = Some(name.to_owned());
                    i += 1;
                } else {
                    // --project my_proj
                    if i + 1 >= args.len() {
                        return Err(anyhow!("Missing project name after --project"));
                    }
                    config.project = Some(args[i + 1].clone());
                    i += 2;
                }
            }
            // If this doesn't start with -- and we don't have a project_name yet,
            // treat it as the project name
            _ if !arg.starts_with("--") && config.project_name.is_none() => {
                config.project_name = Some(arg.clone());
                i += 1;
            }
            // Otherwise, this and all remaining args are for the application
            _ => {
                // Collect this and all remaining args as app_args
                config.app_args.extend(args[i..].iter().cloned());
                break;
            }
        }
    }

    Ok(config)
}
