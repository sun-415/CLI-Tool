use crate::model::{Kind, ParseError, Record};

pub fn parse_kind(s: &str) -> Option<Kind> {
    match s.trim().to_lowercase().as_str() {
        "workout" => Some(Kind::Workout),
        "meal" => Some(Kind::Meal),
        "sleep" => Some(Kind::Sleep),
        _ => None,
    }
}

pub fn parse_line(line: &str) -> Result<Record, ParseError> {
    let trimmed = line.trim();

    if trimmed.is_empty() {
        return Err(ParseError::EmptyLine);
    }

    let parts: Vec<&str> = trimmed.split(',').collect();

    if parts.len() != 3 {
        return Err(ParseError::WrongFieldCount);
    }

    let date_raw = parts[0].trim().to_string();
    let kind_raw = parts[1].trim().to_string();
    let amount_raw = parts[2].trim().to_string();

    if parse_kind(&kind_raw).is_none() {
        return Err(ParseError::InvalidKind);
    }

    Ok(Record {
        date_raw,
        kind_raw,
        amount_raw,
    })
}