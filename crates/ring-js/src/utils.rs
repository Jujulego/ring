use semver::VersionReq;
use tracing::warn;
use ring_utils::{NormalizedPath, Requirement};

pub fn parse_js_requirement(requirement: &str, base_path: &NormalizedPath) -> Requirement {
    match requirement.split_once(':') {
        None if requirement == "*" => Requirement::Any,
        None => Requirement::Version(VersionReq::parse(requirement).unwrap()), // TODO handle this error
        Some(("file", path)) => Requirement::Path(base_path.join(path)),
        _ => {
            warn!("Unknown js requirement: {}", requirement);
            Requirement::default()
        }
    }
}