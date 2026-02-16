use anyhow::{Result, anyhow};
use std::env;

use crate::helpers::{print_banner, print_help};

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct CliArgs {
    pub test: bool,
    pub build: bool,
    pub release: bool,
    pub release_bin: Option<Option<String>>,
    pub project: Option<String>,
    pub project_name: Option<String>,
    pub project_args: Vec<String>,
}

pub fn parse_args_from<I, S>(iter: I) -> Result<CliArgs>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let mut tokens = iter.into_iter().map(Into::into).peekable();

    let mut parsed_args = CliArgs::default();
    let mut project_args = Vec::<String>::new();
    let mut stop_option_parsing = false;

    while let Some(token) = tokens.next() {
        if stop_option_parsing {
            project_args.push(token);
            continue;
        }

        if token == "--" {
            stop_option_parsing = true;
            continue;
        }

        match token.as_str() {
            "-h" | "--help" => {
                print_help();
                std::process::exit(0);
            }
            "-V" | "--version" => {
                print_banner();
                std::process::exit(0);
            }
            "--test" => parsed_args.test = true,
            "--build" => parsed_args.build = true,
            "--release" => parsed_args.release = true,

            arg if arg == "--release-bin" || arg.starts_with("--release-bin=") => {
                if let Some(dest) = arg.strip_prefix("--release-bin=") {
                    parsed_args.release_bin = if dest.is_empty() {
                        Some(None)
                    } else {
                        Some(Some(dest.into()))
                    };
                } else if let Some(next) = tokens.peek() {
                    if !next.starts_with("--") {
                        parsed_args.release_bin = Some(Some(tokens.next().unwrap()));
                    } else {
                        parsed_args.release_bin = Some(None);
                    }
                } else {
                    parsed_args.release_bin = Some(None);
                }
            }

            arg if arg == "--project" || arg.starts_with("--project=") => {
                if let Some(name) = arg.strip_prefix("--project=") {
                    if name.is_empty() {
                        return Err(anyhow!("Missing project name after --project"));
                    }
                    parsed_args.project = Some(name.into());
                } else if let Some(name) = tokens.next() {
                    parsed_args.project = Some(name);
                } else {
                    return Err(anyhow!("Missing project name after --project"));
                }
            }

            other => project_args.push(other.to_owned()),
        }
    }

    if parsed_args.project.is_none() {
        if let Some(first) = project_args.first() {
            if !first.starts_with("--") {
                parsed_args.project_name = Some(project_args.remove(0));
            }
        }
    }

    parsed_args.project_args = project_args;

    Ok(parsed_args)
}

pub fn parse_args() -> Result<CliArgs> {
    parse_args_from(env::args().skip(1))
}
