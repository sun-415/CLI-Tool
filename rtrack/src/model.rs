use chrono::NaiveDate;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Record {
    pub date_raw: String,
    pub kind_raw: String,
    pub amount_raw: String,
}

#[derive(Debug, Clone)]
pub enum ParseError {
    EmptyLine,
    WrongFieldCount,
    InvalidKind,
    InvalidAmount,
    InvalidDate,
}

#[derive(Debug, Clone)]
pub enum Kind {
    Workout,
    Meal,
    Sleep,
}

#[derive(Debug, Clone)]
pub struct Entry {
    pub date: NaiveDate,
    pub kind: Kind,
    pub amount: u32,
}

impl ParseError {
    pub fn reason_key(&self) -> &'static str {
        match self {
            ParseError::EmptyLine => "empty_line",
            ParseError::WrongFieldCount => "wrong_field_count",
            ParseError::InvalidKind => "invalid_kind",
            ParseError::InvalidAmount => "invalid_amount",
            ParseError::InvalidDate => "invalid_date",
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::EmptyLine => write!(f, "line is empty"),
            ParseError::WrongFieldCount => write!(f, "line does not have exactly 3 fields"),
            ParseError::InvalidKind => write!(f, "kind must be workout, meal, or sleep"),
            ParseError::InvalidAmount => write!(f, "amount must be a non-negative integer"),
            ParseError::InvalidDate => {
                write!(f, "date must be a valid date in exact YYYY-MM-DD format")
            }
        }
    }
}

impl Error for ParseError {}