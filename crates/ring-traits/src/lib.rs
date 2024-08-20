mod detector;
mod manifest;
mod project;
mod scope;
mod tagged;

pub use detector::{Detector, DetectAs};
pub use manifest::Manifest;
pub use project::{Project, ProjectDetector};
pub use scope::{Scope, ScopeDetector};
pub use tagged::{Tagged, TaggedDetector};
