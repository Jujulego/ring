mod ring_task;
mod ring_task_detector;

pub use crate::ring_task::RingTask;
pub use crate::ring_task_detector::RingTaskDetector;
use ring_core_modules::{Module, RegistryRef};
use ring_core_tasks::DetectProcessTask;
use std::rc::Rc;

#[derive(Clone)]
pub struct RingModule {
    ring_task_detector: Rc<RingTaskDetector>,
}

impl RingModule {
    /// Creates a new instance of RingModule
    #[inline]
    pub fn new(registry: Rc<RegistryRef>) -> Self {
        RingModule {
            ring_task_detector: Rc::new(RingTaskDetector::new(registry)),
        }
    }

    /// Returns a pointer on RingTaskDetector
    #[inline]
    pub fn ring_task_detector(&self) -> Rc<RingTaskDetector> {
        self.ring_task_detector.clone()
    }
}

impl Module for RingModule {
    #[inline]
    fn task_detectors(&self) -> Vec<Rc<dyn DetectProcessTask>> {
        vec![self.ring_task_detector()]
    }
}