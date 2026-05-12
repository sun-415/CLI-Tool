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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_kind_accepts_valid_kinds_case_insensitively() {
        let cases = [
            ("workout", Some(Kind::Workout)),
            (" meal ", Some(Kind::Meal)),
            ("SLEEP", Some(Kind::Sleep)),
        ];

        for (input, expected) in cases {
            assert_eq!(parse_kind(input), expected);
        }
    }

    #[test]
    fn parse_kind_rejects_unknown_kinds() {
        let cases = ["running", "bike", ""];

        for input in cases {
            assert_eq!(parse_kind(input), None);
        }
    }

    #[test]
    fn parse_line_rejects_wrong_field_count() {
        let result = parse_line("2026-01-05,meal");

        assert_eq!(result.unwrap_err(), ParseError::WrongFieldCount);
    }

    #[test]
    fn parse_line_rejects_invalid_kind() {
        let result = parse_line("2026-01-07,running,25");

        assert_eq!(result.unwrap_err(), ParseError::InvalidKind);
    }

    #[test]
    fn parse_line_trims_spaces_around_fields() {
        let record = parse_line(" 2026-01-01 , workout , 45 ").unwrap();

        assert_eq!(record.date_raw, "2026-01-01");
        assert_eq!(record.kind_raw, "workout");
        assert_eq!(record.amount_raw, "45");
    }
}
