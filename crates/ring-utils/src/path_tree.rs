use std::collections::{HashMap, VecDeque};
use std::ffi::OsString;
use std::path::{Component, Path};

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
        match path.components().next() {
            Some(Component::Prefix(prefix)) => self.prefixes.get(prefix.as_os_str()),
            Some(_) => unreachable!(),
            None => unreachable!()
        }
    }

    fn root_mut(&mut self, path: &Path) -> &mut PathNode<T> {
        match path.components().next() {
            Some(Component::Prefix(prefix)) => {
                self.prefixes.entry(prefix.as_os_str().to_os_string())
                    .or_default()
            },
            Some(_) => unreachable!(),
            None => unreachable!()
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
            let mut stack = VecDeque::from([root]);
    
            for component in path.components() {
                let parent = stack.pop_back().unwrap();
                
                match component {
                    Component::ParentDir => {
                        if stack.is_empty() { // <= then "parent" is the root
                            stack.push_back(parent);
                        }
                    }
                    Component::Normal(name) => {
                        stack.push_back(parent);

                        if let Some(node) = parent.children.get(name) {
                            stack.push_back(node);
                        } else {
                            return None;
                        }
                    }
                    _ => {
                        stack.push_back(parent);
                    },
                }
            }
            
            stack.pop_back()
        } else {
            None
        }
    }

    fn node_mut(&mut self, path: &Path) -> &mut PathNode<T> {
        let root = get_root_mut!(self, path);
        let mut stack = VecDeque::from([root]);

        for component in path.components() {
            let parent = stack.pop_back().unwrap();

            match component {
                Component::ParentDir => {
                    if stack.is_empty() { // <= then "parent" is the root
                        stack.push_back(parent);
                    }
                }
                Component::Normal(name) => {
                    let node = parent.children.entry(name.to_os_string()).or_default();

                    stack.push_back(node);
                }
                _ => {
                    stack.push_back(parent);
                },
            }
        }

        stack.pop_back().unwrap()
    }

    pub fn get(&self, path: &Path) -> Option<&T> {
        assert!(path.is_absolute(), "PathTree keys must be absolute paths");
        self.node(path).and_then(|n| n.data.as_ref())
    }

    pub fn set(&mut self, path: &Path, value: T) {
        assert!(path.is_absolute(), "PathTree keys must be absolute paths");
        self.node_mut(path).data = Some(value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(windows)]
    macro_rules! absolute_path {
        ($path:expr) => { std::path::PathBuf::from(r"C:\".to_owned() + "test") }
    }

    #[cfg(not(windows))]
    macro_rules! absolute_path {
        ($path:expr) => { std::path::PathBuf::from("/".to_owned() + "test") }
    }

    #[test]
    fn it_should_return_none_on_empty_tree() {
        let tree: PathTree<&str> = PathTree::new();

        assert_eq!(tree.get(&absolute_path!("test")), None);
    }

    #[test]
    fn it_should_return_stored_value() {
        let mut tree = PathTree::new();

        tree.set(&absolute_path!("test"), "test");

        assert_eq!(tree.get(&absolute_path!("test")), Some(&"test"));
    }
}