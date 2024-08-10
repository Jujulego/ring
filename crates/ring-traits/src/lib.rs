mod detector;
mod manifest;
mod optional_result;
mod project;
mod scope;
mod tagged;

pub use detector::Detector;
pub use manifest::Manifest;
pub use optional_result::OptionalResult;
pub use project::{Project, ProjectDetector};
pub use scope::{Scope, ScopeDetector};
pub use tagged::{Tagged, TaggedDetector};
