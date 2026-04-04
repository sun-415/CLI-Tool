use std::env;
use std::fs;
use std::process;

use rtrack::report::{build_report, print_report};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: rtrack <input_file>");
        process::exit(1);
    }

    let path = &args[1];

    let contents = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error: could not read file '{path}': {e}");
            process::exit(1);
        }
    };

    let report = build_report(&contents);
    print_report(&report);
}