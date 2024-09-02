use semver::VersionReq;
use crate::NormalizedPathBuf;

#[derive(Debug, Eq, PartialEq)]
pub enum Requirement {
    Any,
    Path(NormalizedPathBuf),
    Version(VersionReq),
}