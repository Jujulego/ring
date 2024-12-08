use std::rc::Rc;
use sysinfo::{Pid, Process};
use ring_code_unit::CodeUnit;
use ring_tag::Tagged;

////////////////////////////////////////////////////////////////////////////////
// Process Unit
////////////////////////////////////////////////////////////////////////////////

/// Defines a process unit.
pub trait ProcessUnit: Tagged {
    /// Process command line
    fn cmd(&self) -> Vec<&str>;

    /// Process id
    fn pid(&self) -> &Pid;

    /// Code unit running in this process
    fn running_code_unit(&self) -> Option<Rc<dyn CodeUnit>>;

    /// Should process be hidden
    fn should_hide(&self) -> bool {
        false
    }
}

////////////////////////////////////////////////////////////////////////////////
// Process Unit Detector
////////////////////////////////////////////////////////////////////////////////

/// Object detecting a process unit
pub trait ProcessUnitDetector {
    fn detect(&self, pid: &Pid, process: &Process) -> anyhow::Result<Option<Rc<dyn ProcessUnit>>>;
}