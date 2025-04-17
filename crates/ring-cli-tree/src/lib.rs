use std::collections::{BTreeSet, HashMap};
use std::fmt::Display;

/// Formats given data as a tree
#[derive(Clone, Debug)]
pub struct Tree {
    roots: BTreeSet<String>,
    children: HashMap<String, BTreeSet<String>>,
}

impl Tree {
    /// Creates a new empty tree
    #[inline]
    pub fn new() -> Self {
        Tree {
            roots: BTreeSet::new(),
            children: HashMap::new(),
        }
    }

    /// Add a new root to the tree
    pub fn add_root(&mut self, key: String) {
        self.roots.insert(key);
    }

    /// Add a new node to the tree
    pub fn add_node(&mut self, key: String, parent: String) {
        self.roots.remove(&key);
        self.roots.insert(parent.clone());

        self.children.entry(parent)
            .or_default().insert(key);
    }

    /// Returns an iterator over tree line items
    #[inline]
    pub fn iter(&self) -> TreeIter<'_> {
        TreeIter::new(self)
    }
}

impl Default for Tree {
    #[inline]
    fn default() -> Self {
        Tree::new()
    }
}

impl<'a> IntoIterator for &'a Tree {
    type Item = TreeNode<'a>;
    type IntoIter = TreeIter<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[derive(Clone, Debug)]
pub struct TreeIter<'a> {
    tree: &'a Tree,
    stack: Vec<(&'a String, Vec<bool>)>,
}

impl<'a> TreeIter<'a> {
    fn new(tree: &'a Tree) -> Self {
        TreeIter {
            tree,
            stack: Vec::from_iter(tree.roots.iter()
                .rev()
                .map(|key| (key, vec![]))
            ),
        }
    }
}

impl<'a> Iterator for TreeIter<'a> {
    type Item = TreeNode<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let (key, branches) = self.stack.pop()?;

        if let Some(children) = self.tree.children.get(key) {
            self.stack.extend(
                children.iter()
                    .rev()
                    .enumerate()
                    .map(|(idx, k)| {
                        let mut child_branches = branches.clone();
                        child_branches.push(idx == 0);

                        (k, child_branches)
                    })
            );
        }

        Some(TreeNode { key, branches })
    }
}

pub struct TreeNode<'a> {
    key: &'a String,
    branches: Vec<bool>,
}

impl Display for TreeNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.branches.is_empty() {
            return write!(f, "\u{25CF} {}", self.key);
        }

        for &is_last in self.branches[..self.branches.len() - 1].iter() {
            if is_last {
                write!(f, "  ")?;
            } else {
                write!(f, "\u{2502} ")?;
            }
        }

        if self.branches[self.branches.len() - 1] {
            write!(f, "\u{2514}\u{2574}{}", self.key)
        } else {
            write!(f, "\u{251C}\u{2574}{}", self.key)
        }
    }
}