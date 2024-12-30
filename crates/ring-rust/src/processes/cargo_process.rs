use crate::language::RUST_COLOR;
use crate::CargoCrate;
use ring_color::ColorLabel;
use ring_core::{CodeUnit, ProcessUnit};
use ring_tag::{Tag, Tagged};
use std::rc::Rc;
use sysinfo::Pid;

#[derive(Clone, Debug)]
pub struct CargoProcess {
    pid: Pid,
    arguments: Vec<String>,
    cargo_crate: Option<Rc<CargoCrate>>,
}

impl CargoProcess {
    pub fn new(pid: Pid, arguments: Vec<String>, cargo_crate: Option<Rc<CargoCrate>>) -> CargoProcess {
        CargoProcess { pid, arguments, cargo_crate }
    }

    pub fn cargo_crate(&self) -> Option<&Rc<CargoCrate>> {
        self.cargo_crate.as_ref()
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
        self.cargo_crate.clone()
            .map(|crt| crt as Rc<dyn CodeUnit>)
    }
}

impl Tagged for CargoProcess {
    fn tags(&self) -> Vec<Tag> {
        vec![Tag::from("rust:cargo").with_color(ColorLabel::Red, RUST_COLOR)]
    }
}
