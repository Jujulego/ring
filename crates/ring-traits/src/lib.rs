mod detector;
mod project;
mod scope;
mod tagged;

pub use detector::Detector;
pub use project::{Project, ProjectDetector};
pub use scope::{Scope, ScopeDetector};
pub use tagged::{Tagged, TaggedDetector};