use std::env;
use std::fs;
use std::process;

fn is_valid_line(line: &str) -> bool {
    // Reject empty or whitespace only lines
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return false;
    }

    // Split by comma and valid only if exactly 3 fields 
    let field_count = trimmed.split(',').count();
    field_count == 3
}

fn main() {
    // Collect args where args[0] is the program name, args[1] should be the file path
    let args: Vec<String> = env::args().collect();

    // Accept exactly one positional argument which is the input file path
    if args.len() != 2 {
        eprintln!("Usage: rtrack <input_file>");
        process::exit(1);
    }

    let path = &args[1];

    // Read the whole file into a string
    let contents = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error: could not read file '{path}': {e}");
            process::exit(1);
        }
    };

    // Week 1: Count number of lines (records), including empty ones for now
    let total_records = contents.lines().count();

    println!("Total records: {}", total_records);
}
