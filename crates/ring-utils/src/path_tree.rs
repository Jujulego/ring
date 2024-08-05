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
enum NodeGetResult<'n, 'c, T> {
    Found(&'n PathNode<T>),
    Missing,
    Back(Components<'c>),
}

#[derive(Debug)]
enum NodeSetResult<'c, T> {
    Stored,
    Back(Components<'c>, T),
}

impl<T> PathNode<T> {
    fn get<'c>(&self, mut components: Components<'c>) -> NodeGetResult<'_, 'c, T> {
        loop {
            match components.next() {
                Some(Component::Normal(name)) => {
                    if let Some(next) = self.children.get(name) {
                        match next.get(components) {
                            NodeGetResult::Back(c) => {
                                components = c;
                            },
                            result => break result,
                        }
                    } else {
                        break NodeGetResult::Missing;
                    }
                }
                Some(Component::ParentDir) => break NodeGetResult::Back(components),
                None => break NodeGetResult::Found(self),
                _ => continue,
            }
        }
    }

    fn set<'c>(&mut self, mut components: Components<'c>, mut value: T) -> NodeSetResult<'c, T> {
        loop {
            match components.next() {
                Some(Component::Normal(name)) => {
                    let next = self.children.entry(name.to_os_string()).or_default();

                    match next.set(components, value) {
                        NodeSetResult::Back(c, v) => {
                            components = c;
                            value = v;
                        },
                        NodeSetResult::Stored => break NodeSetResult::Stored
                    }
                }
                Some(Component::ParentDir) => break NodeSetResult::Back(components, value),
                None => {
                    self.data = Some(value);
                    break NodeSetResult::Stored;
                }
                _ => continue,
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct PathTree<T> {
    #[cfg(windows)]
    prefixes: HashMap<OsString, PathNode<T>>,
    #[cfg(not(windows))]
    root: PathNode<T>,
}

impl<T> PathTree<T> {
    pub fn new() -> PathTree<T> {
        #[cfg(windows)]
        return PathTree { prefixes: HashMap::new() };

        #[cfg(not(windows))]
        return PathTree { root: PathNode::default() };
    }

    #[cfg(windows)]
    fn root(&self, path: &Path) -> Option<&PathNode<T>> {
        if let Some(Component::Prefix(prefix)) = path.components().next() {
            self.prefixes.get(prefix.as_os_str())
        } else {
            unreachable!()
        }
    }

    #[cfg(not(windows))]
    fn root(&self) -> &PathNode<T> {
        &self.root
    }

    fn root_mut(&mut self, #[cfg(windows)] path: &Path) -> &mut PathNode<T> {
        #[cfg(windows)]
        if let Some(Component::Prefix(prefix)) = path.components().next() {
            self.prefixes.entry(prefix.as_os_str().to_os_string())
                .or_default()
        } else {
            unreachable!()
        }

        #[cfg(not(windows))]
        &mut self.root
    }

    pub fn get(&self, path: &Path) -> Option<&T> {
        assert!(path.is_absolute(), "PathTree keys must be absolute paths");

        let root = {
            #[cfg(windows)] { self.root(path)? }
            #[cfg(not(windows))] { self.root() }
        };
        let mut components = path.components();

        loop {
            match root.get(components) {
                NodeGetResult::Found(node) => break node.data.as_ref(),
                NodeGetResult::Missing => break None,
                NodeGetResult::Back(c) => {
                    components = c
                },
            }
        }
    }

    pub fn set(&mut self, path: &Path, mut value: T) {
        assert!(path.is_absolute(), "PathTree keys must be absolute paths");

        let root = self.root_mut(#[cfg(windows)] path);
        let mut components = path.components();

        loop {
            match root.set(components, value) {
                NodeSetResult::Stored => break,
                NodeSetResult::Back(c, v) => {
                    components = c;
                    value = v;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::absolute_path;
    use super::*;

    #[test]
    fn it_should_return_none_on_empty_tree() {
        let tree: PathTree<&str> = PathTree::new();

        assert_eq!(tree.get(&absolute_path!("test")), None);
    }

    #[test]
    #[should_panic(expected = "PathTree keys must be absolute paths")]
    fn it_should_panic_if_get_is_called_without_an_absolute_path() {
        let tree: PathTree<&str> = PathTree::new();
        tree.get(Path::new("test"));
    }

    #[test]
    #[should_panic(expected = "PathTree keys must be absolute paths")]
    fn it_should_panic_if_set_is_called_without_an_absolute_path() {
        let mut tree = PathTree::new();

        tree.set(Path::new("test"), "failed");
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

        assert_eq!(tree.get(&absolute_path!("test/life/42")), Some(&"failed"));
        assert_eq!(tree.get(&absolute_path!("test/././life/./42")), Some(&"failed"));

        tree.set(&absolute_path!("test/././life/./42"), "ok");

        assert_eq!(tree.get(&absolute_path!("test/life/42")), Some(&"ok"));
        assert_eq!(tree.get(&absolute_path!("test/././life/./42")), Some(&"ok"));
    }

    #[test]
    fn it_should_handle_path_with_parent_dirs() {
        let mut tree = PathTree::new();

        tree.set(&absolute_path!("test/life/42"), "failed");

        assert_eq!(tree.get(&absolute_path!("test/life/42")), Some(&"failed"));
        assert_eq!(tree.get(&absolute_path!("test/../test/life/../../test/life/42")), Some(&"failed"));

        tree.set(&absolute_path!("test/../test/life/../../test/life/42"), "ok");

        assert_eq!(tree.get(&absolute_path!("test/life/42")), Some(&"ok"));
        assert_eq!(tree.get(&absolute_path!("test/../test/life/../../test/life/42")), Some(&"ok"));
    }

    #[test]
    fn it_should_handle_deep_parent_dirs() {
        let mut tree = PathTree::new();

        tree.set(&absolute_path!("test/life/42"), "failed");

        assert_eq!(tree.get(&absolute_path!("test/life/42")), Some(&"failed"));
        assert_eq!(tree.get(&absolute_path!("test/../../../test/life/42")), Some(&"failed"));

        tree.set(&absolute_path!("test/../../../test/life/42"), "ok");

        assert_eq!(tree.get(&absolute_path!("test/life/42")), Some(&"ok"));
        assert_eq!(tree.get(&absolute_path!("test/../../../test/life/42")), Some(&"ok"));
    }
}