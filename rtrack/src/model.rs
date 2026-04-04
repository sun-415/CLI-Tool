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
    pub date: String,
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