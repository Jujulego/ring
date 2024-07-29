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

#[cfg(not(windows))]
#[derive(Debug)]
pub struct PathTree<T> {
    root: PathNode<T>
}

#[cfg(not(windows))]
impl<T> PathTree<T> {
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

impl<T> PathTree<T> {
    pub fn get(&self, path: &Path) -> Option<&T> {
        assert!(path.is_absolute(), "PathTree keys must be absolute paths");

        if let Some(root) = get_root!(self, path) {
            let mut stack = VecDeque::new();
    
            for component in path.components() {
                let current = stack.iter().last().unwrap_or(&root);
                
                match component {
                    Component::ParentDir => {
                        stack.pop_back();
                    }
                    Component::Normal(name) => {
                        if let Some(node) = current.children.get(name) {
                            stack.push_back(node);
                        } else {
                            return None;
                        }
                    }
                    _ => continue,
                }
            }
            
            if let Some(&node) = stack.iter().last() {
                return node.data.as_ref();
            }
        }

        None
    }
}
