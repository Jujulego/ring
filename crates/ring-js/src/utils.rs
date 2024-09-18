use semver::VersionReq;
use tracing::warn;
use ring_utils::{NormalizedPath, Requirement};

pub fn parse_js_requirement(requirement: &str, base_path: &NormalizedPath) -> anyhow::Result<Requirement> {
    Ok(match requirement.split_once(':') {
        None if requirement == "*" => Requirement::Any,
        None => Requirement::Version(VersionReq::parse(requirement)?),
        Some(("file", path)) => Requirement::Path(base_path.join(path)),
        _ => {
            warn!("Unknown js requirement: {}", requirement);
            Requirement::default()
        }
    })
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use ring_utils::Normalize;
    use super::*;
    
    #[test]
    fn it_should_return_parsed_requirement() {
        let path = Path::new("/test/foo").normalize();
        
        assert_eq!(parse_js_requirement("*", &path).unwrap(), Requirement::Any);
        assert_eq!(parse_js_requirement("^1.0.0", &path).unwrap(), Requirement::Version(VersionReq::parse("^1.0.0").unwrap()));
        assert_eq!(parse_js_requirement("file:../bar", &path).unwrap(), Requirement::Path(Path::new("/test/bar").normalize()));
    }
    
    #[test]
    fn it_should_return_default_for_unsupported_protocol() {
        let path = Path::new("/test/foo").normalize();
        
        assert_eq!(parse_js_requirement("toto:foo", &path).unwrap(), Requirement::default());
    }
}