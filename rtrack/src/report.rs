use std::collections::BTreeMap;

use crate::model::ParseError;

#[derive(Debug, Default)]
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

    pub fn add_delta(&mut self, delta: Report) {
        self.total_records += delta.total_records;
        self.valid_records += delta.valid_records;
        self.rejected_records += delta.rejected_records;

        for (reason, count) in delta.rejection_reasons {
            let total = self.rejection_reasons.entry(reason).or_insert(0);
            *total += count;
        }
    }
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
