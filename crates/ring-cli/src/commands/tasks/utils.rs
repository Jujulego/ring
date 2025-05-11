use std::iter::FusedIterator;
use sysinfo::{Process, System};

pub struct ProcessAncestors<'a> {
    sys: &'a System,
    process: Option<&'a Process>,
}

impl<'a> ProcessAncestors<'a> {
    pub fn new(sys: &'a System, process: &'a Process) -> Self {
        Self {
            sys,
            process: Some(process),
        }
    }
}

impl<'a> Iterator for ProcessAncestors<'a> {
    type Item = &'a Process;

    fn next(&mut self) -> Option<Self::Item> {
        let process = self.process?;
        self.process = process.parent().and_then(|pid| self.sys.process(pid));

        Some(process)
    }
}

impl FusedIterator for ProcessAncestors<'_> {}