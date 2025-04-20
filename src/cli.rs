use anyhow::{anyhow, Result};
use std::env;

use crate::helpers::{print_banner, print_help};

pub struct CliArgs {
    pub build: bool,
    pub release: bool,
    pub release_bin: Option<Option<String>>,
    pub project: Option<String>,
    pub project_name: Option<String>,
    pub project_args: Vec<String>,
}

impl Default for CliArgs {
    fn default() -> Self {
        Self {
            build: false,
            release: false,
            release_bin: None,
            project: None,
            project_name: None,
            project_args: Vec::new(),
        }
    }
}

pub fn parse_args() -> Result<CliArgs> {
    let mut all_args = env::args().skip(1).peekable();

    let mut cli_args = CliArgs::default();
    let mut project_args = Vec::<String>::new();

    while let Some(arg) = all_args.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                print_help();
                std::process::exit(0);
            }
            "-V" | "--version" => {
                print_banner();
                std::process::exit(0);
            }
            "--build" => cli_args.build = true,
            "--release" => cli_args.release = true,

            arg if arg == "--release-bin" || arg.starts_with("--release-bin=") => {
                if let Some(dest) = arg.strip_prefix("--release-bin=") {
                    cli_args.release_bin = if dest.is_empty() {
                        Some(None)
                    } else {
                        Some(Some(dest.into()))
                    };
                } else if let Some(next) = all_args.peek() {
                    if !next.starts_with("--") {
                        cli_args.release_bin = Some(Some(all_args.next().unwrap()));
                    } else {
                        cli_args.release_bin = Some(None);
                    }
                } else {
                    cli_args.release_bin = Some(None);
                }
            }

            arg if arg == "--project" || arg.starts_with("--project=") => {
                if let Some(name) = arg.strip_prefix("--project=") {
                    if name.is_empty() {
                        return Err(anyhow!("Missing project name after --project"));
                    }
                    cli_args.project = Some(name.into());
                } else if let Some(name) = all_args.next() {
                    cli_args.project = Some(name);
                } else {
                    return Err(anyhow!("Missing project name after --project"));
                }
            }

            other => project_args.push(other.to_owned()),
        }
    }

    if cli_args.project.is_none() {
        if let Some(first) = project_args.first() {
            if !first.starts_with("--") {
                cli_args.project_name = Some(project_args.remove(0));
            }
        }
    }

    cli_args.project_args = project_args;

    Ok(cli_args)
}
