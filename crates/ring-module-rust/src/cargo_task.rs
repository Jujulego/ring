use std::rc::Rc;
use ring_core_tasks::Task;
use ring_core_units::Unit;
use crate::cargo_crate::CargoCrate;

/// A cargo process
pub struct CargoTask {
    cargo_crate: Option<Rc<CargoCrate>>,
}

impl Task for CargoTask {
    fn kind(&self) -> &str {
        "cargo"
    }

    fn working_unit(&self) -> Option<Rc<dyn Unit>> {
        self.cargo_crate.as_ref()
            .map(|pt| pt.clone() as Rc<dyn Unit>)
    }

    #[cfg(feature = "crossterm")]
    fn style(&self) -> crossterm::style::ContentStyle {
        crossterm::style::ContentStyle {
            foreground_color: Some(crossterm::style::Color::Rgb { r: 0xe3, g: 0x3b, b: 0x26 }),
            ..Default::default()
        }
    }
}