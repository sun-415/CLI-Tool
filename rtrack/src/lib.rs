pub mod model;
pub mod parse;
pub mod processor;
pub mod report;
pub mod validate;

use crate::parse::parse_line;
use crate::processor::Processor;
use crate::report::Report;
use crate::validate::validate_record;

pub fn process_str(input: &str, processor: &mut impl Processor) -> Report {
    let mut report = Report::new();

    for line in input.lines() {
        report.total_records += 1;

        match parse_line(line).and_then(validate_record) {
            Ok(entry) => {
                let delta = processor.process(entry);
                report.add_delta(delta);
            }
            Err(error) => {
                report.add_rejection(error);
            }
        }
    }

    report
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::processor::CountingProcessor;

    #[test]
    fn process_str_counts_valid_and_rejected_records() {
        let input = "\
2026-01-01,workout,45
2026-02-30,meal,100
2026-01-03,sleep,480";

        let mut processor = CountingProcessor::new();
        let report = process_str(input, &mut processor);

        assert_eq!(report.total_records, 3);
        assert_eq!(report.valid_records, 2);
        assert_eq!(report.rejected_records, 1);
        assert_eq!(report.rejection_reasons.get("invalid_date"), Some(&1));
    }
}
