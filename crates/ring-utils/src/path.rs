use std::ffi::OsStr;
use std::path::{Component, Path, PrefixComponent};

#[derive(Debug, PartialEq, Eq)]
pub enum AbsoluteNormalizedComponent<'p> {
    Prefix(PrefixComponent<'p>),
    RootDir,
    Normal(&'p OsStr)
}

pub type ANC<'p> = AbsoluteNormalizedComponent<'p>;

impl<'p> PartialEq<Component<'p>> for AbsoluteNormalizedComponent<'p> {
    fn eq(&self, other: &Component<'p>) -> bool {
        match (self, other) {
            (ANC::Prefix(a), Component::Prefix(b)) => a == b,
            (ANC::RootDir, Component::RootDir) => true,
            (ANC::Normal(a), Component::Normal(b)) => *a == *b,
            (_, _) => false,
        }
    }
}

pub fn normalize(path: &Path) -> Vec<ANC> {
    assert!(path.is_absolute(), "normalize only works on absolute paths");
    let mut components = Vec::new();

    for component in path.components() {
        match component {
            Component::Prefix(prefix) => components.push(ANC::Prefix(prefix)),
            Component::RootDir => components.push(ANC::RootDir),
            Component::CurDir => continue,
            Component::ParentDir => {
                match components.last() {
                    Some(ANC::RootDir | ANC::Prefix(_)) | None => continue,
                    Some(_) => { components.pop(); },
                }
            },
            Component::Normal(str) => components.push(ANC::Normal(str)),
        }
    }

    components
}

#[cfg(test)]
mod tests {
    use crate::absolute_path;
    use super::*;

    #[test]
    fn it_should_normalize_given_path() {
        assert_eq!(
            normalize(&absolute_path!("/./test/../life/test/toto/../../42")),
            absolute_path!("/life/42").components().collect::<Vec<_>>()
        );
    }
}