use std::rc::Rc;
use crate::{CodeUnitDetector, ProcessUnitDetector};

pub trait RingModule {
    fn code_unit_detector(&self) -> Vec<Rc<dyn CodeUnitDetector>>;
    fn process_unit_detector(&self) -> Vec<Rc<dyn ProcessUnitDetector>>;
}