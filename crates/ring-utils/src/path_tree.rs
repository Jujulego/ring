use crate::{NormalizedComponent, NormalizedPath};
use std::collections::HashMap;
use std::ffi::OsString;
use std::fmt::Debug;

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

#[derive(Debug, Default)]
pub struct PathTree<T> {
    prefixes: HashMap<OsString, PathNode<T>>,
    root: PathNode<T>,
}

impl<T> PathTree<T> {
    pub fn new() -> PathTree<T> {
        PathTree {
            prefixes: HashMap::new(),
            root: PathNode::default()
        }
    }

    fn root(&self, path: &NormalizedPath) -> Option<&PathNode<T>> {
        if let Some(prefix) = path.prefix() {
            self.prefixes.get(prefix.as_os_str())
        } else {
            Some(&self.root)
        }
    }

    fn root_mut(&mut self, path: &NormalizedPath) -> &mut PathNode<T> {
        if let Some(prefix) = path.prefix() {
            self.prefixes.entry(prefix.as_os_str().to_os_string())
                .or_default()
        } else {
            &mut self.root
        }
    }

    pub fn get(&self, path: &NormalizedPath) -> Option<&T> {
        let mut node = self.root(path)?;
        
        for component in path.components() {
            if let NormalizedComponent::Normal(name) = component {
                node = node.children.get(name)?
            }
        }

        node.data.as_ref()
    }

    pub fn get_mut(&mut self, path: &NormalizedPath) -> Option<&mut T> {
        let mut node = self.root_mut(path);

        for component in path.components() {
            if let NormalizedComponent::Normal(name) = component {
                node = node.children.get_mut(name)?;
            }
        }

        node.data.as_mut()
    }

    pub fn set(&mut self, path: &NormalizedPath, value: T) {
        let mut node = self.root_mut(path);

        for component in path.components() {
            if let NormalizedComponent::Normal(name) = component {
                node = node.children.entry(name.to_os_string()).or_default();
            }
        }
        
        node.data = Some(value);
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use super::*;
    use crate::Normalize;

    #[test]
    fn it_should_return_none_on_empty_tree() {
        let mut tree: PathTree<&str> = PathTree::new();
        let path = Path::new("/test").normalize();

        assert_eq!(tree.get(&path), None);
        assert_eq!(tree.get_mut(&path), None);
    }

    #[test]
    fn it_should_return_stored_value() {
        let mut tree = PathTree::new();
        let path = Path::new("/test/life/42").normalize();

        tree.set(&path, "ok");
        assert_eq!(tree.get(&path), Some(&"ok"));
    }

    #[test]
    fn it_should_mutate_stored_value() {
        let mut tree = PathTree::new();
        let path = Path::new("/test/life/42").normalize();

        tree.set(&path, "failed");
        *tree.get_mut(&path).unwrap() = "ok";
        assert_eq!(tree.get(&path), Some(&"ok"));
    }
}