use chrono::NaiveDate;

use crate::model::{Entry, ParseError, Record};
use crate::parse::parse_kind;

fn is_strict_yyyy_mm_dd(date: &str) -> bool {
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
    if !is_strict_yyyy_mm_dd(&record.date_raw) {
        return Err(ParseError::InvalidDate);
    }

    let date = NaiveDate::parse_from_str(&record.date_raw, "%Y-%m-%d")
        .map_err(|_| ParseError::InvalidDate)?;

    let kind = parse_kind(&record.kind_raw).ok_or(ParseError::InvalidKind)?;

    let amount = record
        .amount_raw
        .parse::<u32>()
        .map_err(|_| ParseError::InvalidAmount)?;

    Ok(Entry { date, kind, amount })
}