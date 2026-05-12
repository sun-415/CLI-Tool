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

fn parse_amount(amount: &str) -> Result<u32, ParseError> {
    amount.parse::<u32>().map_err(|_| ParseError::InvalidAmount)
}

pub fn validate_record(record: Record) -> Result<Entry, ParseError> {
    if !is_strict_yyyy_mm_dd(&record.date_raw) {
        return Err(ParseError::InvalidDate);
    }

    let date = NaiveDate::parse_from_str(&record.date_raw, "%Y-%m-%d")
        .map_err(|_| ParseError::InvalidDate)?;

    let kind = parse_kind(&record.kind_raw).ok_or(ParseError::InvalidKind)?;

    let amount = parse_amount(&record.amount_raw)?;

    Ok(Entry { date, kind, amount })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record(date: &str, kind: &str, amount: &str) -> Record {
        Record {
            date_raw: date.to_string(),
            kind_raw: kind.to_string(),
            amount_raw: amount.to_string(),
        }
    }

    #[test]
    fn parse_amount_accepts_non_negative_integers() {
        let cases = [("0", Ok(0)), ("45", Ok(45)), ("650", Ok(650))];

        for (input, expected) in cases {
            assert_eq!(parse_amount(input), expected);
        }
    }

    #[test]
    fn parse_amount_rejects_invalid_values() {
        let cases = ["abc", "-5", "10.5", ""];

        for input in cases {
            assert_eq!(parse_amount(input), Err(ParseError::InvalidAmount));
        }
    }

    #[test]
    fn validate_record_rejects_invalid_calendar_date() {
        let result = validate_record(record("2026-02-30", "meal", "100"));

        assert_eq!(result.unwrap_err(), ParseError::InvalidDate);
    }

    #[test]
    fn validate_record_rejects_invalid_amount() {
        let result = validate_record(record("2026-01-09", "sleep", "-5"));

        assert_eq!(result.unwrap_err(), ParseError::InvalidAmount);
    }
}
