use std::collections::{BTreeSet, HashMap};
use std::fmt::Display;
use std::hash::Hash;

/// Formats given data as a tree
#[derive(Clone, Debug)]
pub struct Tree<K: Clone + Ord + Eq + Hash> {
    roots: BTreeSet<K>,
    children: HashMap<K, BTreeSet<K>>,
}

impl<K: Clone + Ord + Eq + Hash> Tree<K> {
    /// Creates a new empty tree
    #[inline]
    pub fn new() -> Self {
        Tree {
            roots: BTreeSet::new(),
            children: HashMap::new(),
        }
    }

    /// Add a new root to the tree
    pub fn add_root(&mut self, key: K) {
        self.roots.insert(key);
    }

    /// Add a new node to the tree
    pub fn add_node(&mut self, key: K, parent: K) {
        self.roots.remove(&key);
        self.roots.insert(parent.clone());

        self.children.entry(parent)
            .or_default().insert(key);
    }

    /// Returns an iterator over tree line items
    #[inline]
    pub fn iter(&self) -> TreeIter<'_, K> {
        TreeIter::new(self)
    }
}

impl<K: Clone + Ord + Eq + Hash> Default for Tree<K> {
    #[inline]
    fn default() -> Self {
        Tree::new()
    }
}

impl<'a, K: Clone + Ord + Eq + Hash> IntoIterator for &'a Tree<K> {
    type Item = TreeNode<'a, K>;
    type IntoIter = TreeIter<'a, K>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[derive(Clone, Debug)]
pub struct TreeIter<'a, K: Clone + Ord + Eq + Hash> {
    tree: &'a Tree<K>,
    stack: Vec<(&'a K, Vec<bool>)>,
}

impl<'a, K: Clone + Ord + Eq + Hash> TreeIter<'a, K> {
    fn new(tree: &'a Tree<K>) -> Self {
        TreeIter {
            tree,
            stack: Vec::from_iter(tree.roots.iter()
                .rev()
                .map(|key| (key, vec![]))
            ),
        }
    }
}

impl<'a, K: Clone + Ord + Eq + Hash> Iterator for TreeIter<'a, K> {
    type Item = TreeNode<'a, K>;

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

pub struct TreeNode<'a, K: Clone + Ord + Eq + Hash> {
    pub key: &'a K,
    branches: Vec<bool>,
}

impl<K: Clone + Ord + Eq + Hash + Display> Display for TreeNode<'_, K> {
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