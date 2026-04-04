use std::env;
use std::fs;
use std::process;

#[derive(Debug)]
struct Record {
    date_raw: String,
    kind_raw: String,
    amount_raw: String,
}

#[derive(Debug)]
enum ParseError {
    EmptyLine,
    WrongFieldCount,
    InvalidKind,
}

#[derive(Debug)]
enum Kind {
    Workout,
    Meal,
    Sleep,
}

fn parse_kind(s: &str) -> Option<Kind> {
    match s.trim().to_lowercase().as_str() {
        "workout" => Some(Kind::Workout),
        "meal" => Some(Kind::Meal),
        "sleep" => Some(Kind::Sleep),
        _ => None,
    }
}

fn parse_line(line: &str) -> Result<Record, ParseError> {
    let trimmed = line.trim();

    // Reject empty or whitespace-only lines
    if trimmed.is_empty() {
        return Err(ParseError::EmptyLine);
    }

    // Split by comma
    let parts: Vec<&str> = trimmed.split(',').collect();

    // Reject if not exactly 3 fields
    if parts.len() != 3 {
        return Err(ParseError::WrongFieldCount);
    }

    let date_raw = parts[0].trim().to_string();
    let kind_raw = parts[1].trim().to_string();
    let amount_raw = parts[2].trim().to_string();

    // Validate kind for Week 3
    if parse_kind(&kind_raw).is_none() {
        return Err(ParseError::InvalidKind);
    }

    Ok(Record {
        date_raw,
        kind_raw,
        amount_raw,
    })
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

    let mut total_records: usize = 0;
    let mut valid_records: usize = 0;
    let mut rejected_records: usize = 0;

    for line in contents.lines() {
        total_records += 1;

        match parse_line(line) {
            Ok(record) => {
                valid_records += 1;

                // record is currently not used after parsing,
                // but keeping this line shows we successfully parsed it
                let _ = record;
            }
            Err(_error) => {
                rejected_records += 1;
            }
        }
    }

    println!("Total records: {}", total_records);
    println!("Valid records: {}", valid_records);
    println!("Rejected records: {}", rejected_records);

}
