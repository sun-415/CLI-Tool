use crate::model::{Entry, ParseError, Record};
use crate::parse::parse_kind;

fn is_basic_date_format(date: &str) -> bool {
    if date.len() != 10 {
        return false;
    }

    let bytes = date.as_bytes();

    if bytes[4] != b'-' || bytes[7] != b'-' {
        return false;
    }

    for (i, b) in bytes.iter().enumerate() {
        if i == 4 || i == 7 {
            continue;
        }

        if !b.is_ascii_digit() {
            return false;
        }
    }

    true
}

pub fn validate_record(record: Record) -> Result<Entry, ParseError> {
    if !is_basic_date_format(&record.date_raw) {
        return Err(ParseError::InvalidDate);
    }

    let kind = match parse_kind(&record.kind_raw) {
        Some(k) => k,
        None => return Err(ParseError::InvalidKind),
    };

    let amount = match record.amount_raw.parse::<u32>() {
        Ok(n) => n,
        Err(_) => return Err(ParseError::InvalidAmount),
    };

    Ok(Entry {
        date: record.date_raw,
        kind,
        amount,
    })
}