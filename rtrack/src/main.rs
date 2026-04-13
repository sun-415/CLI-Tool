use std::env;
use std::error::Error;
use std::fmt;
use std::fs;
use std::process::ExitCode;

use rtrack::report::{build_report, print_report, Report};

const USAGE: &str = "Usage: rtrack <path> [--strict]";

#[derive(Debug)]
enum AppError {
    InvalidUsage,
    ReadFile {
        path: String,
        source: std::io::Error,
    },
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::InvalidUsage => write!(f, "{USAGE}"),
            AppError::ReadFile { path, source } => {
                write!(f, "Error: could not read file '{path}': {source}")
            }
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::InvalidUsage => None,
            AppError::ReadFile { source, .. } => Some(source),
        }
    }
}

#[derive(Debug)]
struct Cli {
    path: String,
    strict: bool,
}

fn parse_args() -> Result<Cli, AppError> {
    let args: Vec<String> = env::args().skip(1).collect();

    match args.as_slice() {
        [path] => Ok(Cli {
            path: path.clone(),
            strict: false,
        }),
        [path, flag] if flag == "--strict" => Ok(Cli {
            path: path.clone(),
            strict: true,
        }),
        _ => Err(AppError::InvalidUsage),
    }
}

fn run() -> Result<(Report, bool), AppError> {
    let cli = parse_args()?;

    let contents = fs::read_to_string(&cli.path).map_err(|source| AppError::ReadFile {
        path: cli.path.clone(),
        source,
    })?;

    let report = build_report(&contents);
    Ok((report, cli.strict))
}

fn main() -> ExitCode {
    match run() {
        Ok((report, strict)) => {
            print_report(&report);

            if strict && report.rejected_records > 0 {
                ExitCode::from(1)
            } else {
                ExitCode::SUCCESS
            }
        }
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}