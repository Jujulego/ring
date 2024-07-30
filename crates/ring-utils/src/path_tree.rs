use std::collections::HashMap;
use std::ffi::OsString;
use std::fmt::Debug;
use std::path::{Component, Components, Path};

#[derive(Debug)]
struct PathNode<T> {
    children: HashMap<OsString, PathNode<T>>,
    data: Option<T>,
}

impl<T> Default for PathNode<T> {
    fn default() -> Self {
        PathNode {
            children: HashMap::default(),
            data: None,
        }
    }
}

#[derive(Debug)]
enum FollowResult<'n, T> {
    Found(&'n PathNode<T>),
    Missing,
    Back,
}

#[derive(Debug)]
enum FollowResultMut<T> {
    Stored,
    Back(T),
}

impl<T> PathNode<T> {
    fn get(&self, components: &mut Components) -> FollowResult<'_, T> {
        loop {
            match components.next() {
                Some(Component::Normal(name)) => {
                    if let Some(next) = self.children.get(name) {
                        match next.get(components) {
                            FollowResult::Back => continue,
                            result => break result,
                        }
                    } else {
                        break FollowResult::Missing;
                    }
                }
                Some(Component::ParentDir) => break FollowResult::Back,
                None => break FollowResult::Found(self),
                _ => continue,
            }
        }
    }

    fn set(&mut self, components: &mut Components, value: T) -> FollowResultMut<T> {
        let mut value = value;

        loop {
            match components.next() {
                Some(Component::Normal(name)) => {
                    let next = self.children.entry(name.to_os_string()).or_default();

                    match next.set(components, value) {
                        FollowResultMut::Back(v) => value = v,
                        FollowResultMut::Stored => break FollowResultMut::Stored
                    }
                }
                Some(Component::ParentDir) => break FollowResultMut::Back(value),
                None => {
                    self.data = Some(value);
                    break FollowResultMut::Stored;
                }
                _ => continue,
            }
        }
    }
}

#[cfg(windows)]
#[derive(Debug)]
pub struct PathTree<T> {
    prefixes: HashMap<OsString, PathNode<T>>,
}

#[cfg(windows)]
impl<T> PathTree<T> {
    pub fn new() -> PathTree<T> {
        PathTree { prefixes: HashMap::new() }
    }

    fn root(&self, path: &Path) -> Option<&PathNode<T>> {
        if let Some(Component::Prefix(prefix)) = path.components().next() {
            self.prefixes.get(prefix.as_os_str())
        } else {
            unreachable!()
        }
    }

    fn root_mut(&mut self, path: &Path) -> &mut PathNode<T> {
        if let Some(Component::Prefix(prefix)) = path.components().next() {
            self.prefixes.entry(prefix.as_os_str().to_os_string())
                .or_default()
        } else {
            unreachable!()
        }
    }
}

#[cfg(windows)]
macro_rules! get_root {
    ($self:ident, $path:ident) => { $self.root($path) }
}

#[cfg(windows)]
macro_rules! get_root_mut {
    ($self:ident, $path:ident) => { $self.root_mut($path) }
}

#[cfg(not(windows))]
#[derive(Debug)]
pub struct PathTree<T> {
    root: PathNode<T>
}

#[cfg(not(windows))]
impl<T> PathTree<T> {
    pub fn new() -> PathTree<T> {
        PathTree { root: PathNode::default() }
    }

    fn root(&self) -> Option<&PathNode<T>> {
        Some(&self.root)
    }

    fn root_mut(&mut self) -> &mut PathNode<T> {
        &mut self.root
    }
}

#[cfg(not(windows))]
macro_rules! get_root {
    ($self:ident, $path:ident) => { $self.root() }
}

#[cfg(not(windows))]
macro_rules! get_root_mut {
    ($self:ident, $path:ident) => { $self.root_mut() }
}

impl<T> PathTree<T> {
    fn node(&self, path: &Path) -> Option<&PathNode<T>> {
        if let Some(root) = get_root!(self, path) {
            let components = &mut path.components();

            loop {
                match root.get(components) {
                    FollowResult::Found(node) => break Some(node),
                    FollowResult::Missing => break None,
                    FollowResult::Back => continue,
                }
            }
        } else {
            None
        }
    }

    pub fn get(&self, path: &Path) -> Option<&T> {
        assert!(path.is_absolute(), "PathTree keys must be absolute paths");
        self.node(path).and_then(|n| n.data.as_ref())
    }

    pub fn set(&mut self, path: &Path, value: T) {
        assert!(path.is_absolute(), "PathTree keys must be absolute paths");

        let components = &mut path.components();
        let root = get_root_mut!(self, path);
        let mut value = value;

        loop {
            match root.set(components, value) {
                FollowResultMut::Stored => break,
                FollowResultMut::Back(v) => value = v
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(windows)]
    macro_rules! absolute_path {
        ($path:literal) => { std::path::PathBuf::from(r"C:\".to_owned() + $path) }
    }

    #[cfg(not(windows))]
    macro_rules! absolute_path {
        ($path:literal) => { std::path::PathBuf::from("/".to_owned() + $path) }
    }

    #[test]
    fn it_should_return_none_on_empty_tree() {
        let tree: PathTree<&str> = PathTree::new();

        assert_eq!(tree.get(&absolute_path!("test")), None);
    }

    #[test]
    fn it_should_return_stored_value() {
        let mut tree = PathTree::new();

        tree.set(&absolute_path!("test"), "failed");
        tree.set(&absolute_path!("test/life/42"), "ok");
        tree.set(&absolute_path!("test/life"), "failed");

        assert_eq!(tree.get(&absolute_path!("test/life/42")), Some(&"ok"));
    }

    #[test]
    fn it_should_handle_path_with_cur_dirs() {
        let mut tree = PathTree::new();

        tree.set(&absolute_path!("test/life/42"), "failed");
        tree.set(&absolute_path!("test/././life/./42"), "ok");

        assert_eq!(tree.get(&absolute_path!("test/life/42")), Some(&"ok"));
        assert_eq!(tree.get(&absolute_path!("test/././life/./42")), Some(&"ok"));
    }

    #[test]
    fn it_should_handle_path_with_parent_dirs() {
        let mut tree = PathTree::new();

        tree.set(&absolute_path!("test/life/42"), "failed");
        tree.set(&absolute_path!("test/../test/life/../../test/life/42"), "ok");

        assert_eq!(tree.get(&absolute_path!("test/life/42")), Some(&"ok"));
        assert_eq!(tree.get(&absolute_path!("test/../test/life/../../test/life/42")), Some(&"ok"));
    }
}