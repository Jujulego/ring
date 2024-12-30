use crate::language::RUST_COLOR;
use ring_color::ColorLabel;
use ring_core::{CodeUnit, ProcessUnit};
use ring_tag::{Tag, Tagged};
use std::rc::Rc;
use sysinfo::Pid;

#[derive(Clone, Debug)]
pub struct CargoProcess {
    pid: Pid,
    arguments: Vec<String>,
}

impl CargoProcess {
    pub fn new(pid: Pid, arguments: Vec<String>) -> CargoProcess {
        CargoProcess { pid, arguments }
    }
}

impl ProcessUnit for CargoProcess {
    fn cmd(&self) -> Vec<&str> {
        self.arguments.iter().map(|a| a.as_ref()).collect()
    }

    fn pid(&self) -> &Pid {
        &self.pid
    }

    fn running_code_unit(&self) -> Option<Rc<dyn CodeUnit>> {
        None
    }
}

impl Tagged for CargoProcess {
    fn tags(&self) -> Vec<Tag> {
        vec![Tag::from("rust:cargo").with_color(ColorLabel::Red, RUST_COLOR)]
    }
}
