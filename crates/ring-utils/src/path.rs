use std::ffi::OsStr;
use std::path::{Component, Path, PrefixComponent};

#[derive(Debug, PartialEq, Eq)]
pub enum AbsoluteNormalizedComponent<'p> {
    Prefix(PrefixComponent<'p>),
    RootDir,
    Normal(&'p OsStr)
}

pub type Anc<'p> = AbsoluteNormalizedComponent<'p>;

impl<'p> PartialEq<Component<'p>> for AbsoluteNormalizedComponent<'p> {
    fn eq(&self, other: &Component<'p>) -> bool {
        match (self, other) {
            (Anc::Prefix(a), Component::Prefix(b)) => a == b,
            (Anc::RootDir, Component::RootDir) => true,
            (Anc::Normal(a), Component::Normal(b)) => *a == *b,
            (_, _) => false,
        }
    }
}

pub fn normalize(path: &Path) -> Vec<Anc> {
    assert!(path.is_absolute(), "normalize only works on absolute paths");
    let mut components = Vec::new();

    for component in path.components() {
        match component {
            Component::Prefix(prefix) => components.push(Anc::Prefix(prefix)),
            Component::RootDir => components.push(Anc::RootDir),
            Component::CurDir => continue,
            Component::ParentDir => {
                match components.last() {
                    Some(Anc::RootDir | Anc::Prefix(_)) | None => continue,
                    Some(_) => { components.pop(); },
                }
            },
            Component::Normal(str) => components.push(Anc::Normal(str)),
        }
    }

    components
}

#[cfg(test)]
mod tests {
    use crate::absolute_path;
    use super::*;

    #[test]
    fn it_should_normalize_path() {
        assert_eq!(
            normalize(&absolute_path!("test/life/42")),
            absolute_path!("test/life/42").components().collect::<Vec<_>>()
        );
    }

    #[test]
    fn it_should_normalize_path_with_cur_dirs() {
        assert_eq!(
            normalize(&absolute_path!("test/././life/./42")),
            absolute_path!("test/life/42").components().collect::<Vec<_>>()
        );
    }

    #[test]
    fn it_should_normalize_path_with_parent_dirs() {
        assert_eq!(
            normalize(&absolute_path!("test/../test/life/../../test/life/42")),
            absolute_path!("test/life/42").components().collect::<Vec<_>>()
        );
    }

    #[test]
    fn it_should_normalize_deep_parent_dirs() {
        assert_eq!(
            normalize(&absolute_path!("test/../../../test/life/42")),
            absolute_path!("test/life/42").components().collect::<Vec<_>>()
        );
    }
}