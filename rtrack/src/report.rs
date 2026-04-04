use std::collections::BTreeMap;

use crate::model::ParseError;
use crate::parse::parse_line;
use crate::validate::validate_record;

#[derive(Debug)]
pub struct Report {
    pub total_records: u64,
    pub valid_records: u64,
    pub rejected_records: u64,
    pub rejection_reasons: BTreeMap<String, u64>,
}

impl Report {
    pub fn new() -> Self {
        Self {
            total_records: 0,
            valid_records: 0,
            rejected_records: 0,
            rejection_reasons: BTreeMap::new(),
        }
    }

    pub fn add_rejection(&mut self, error: ParseError) {
        self.rejected_records += 1;

        let key = error.reason_key().to_string();
        let count = self.rejection_reasons.entry(key).or_insert(0);
        *count += 1;
    }
}

pub fn build_report(contents: &str) -> Report {
    let mut report = Report::new();

    for line in contents.lines() {
        report.total_records += 1;

        match parse_line(line) {
            Ok(record) => match validate_record(record) {
                Ok(entry) => {
                    report.valid_records += 1;
                    let _ = entry;
                }
                Err(error) => {
                    report.add_rejection(error);
                }
            },
            Err(error) => {
                report.add_rejection(error);
            }
        }
    }

    report
}

pub fn print_report(report: &Report) {
    println!("Total records: {}", report.total_records);
    println!("Valid records: {}", report.valid_records);
    println!("Rejected records: {}", report.rejected_records);
    println!("Rejection reasons:");

    for (reason, count) in &report.rejection_reasons {
        println!("{reason}: {count}");
    }
}