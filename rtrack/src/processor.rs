use crate::model::Entry;
use crate::report::Report;

pub trait Processor {
    fn process(&mut self, entry: Entry) -> Report;
}

#[derive(Debug, Default)]
pub struct CountingProcessor;

impl CountingProcessor {
    pub fn new() -> Self {
        Self
    }
}

impl Processor for CountingProcessor {
    fn process(&mut self, _entry: Entry) -> Report {
        let mut report = Report::new();
        report.valid_records = 1;
        report
    }
}
