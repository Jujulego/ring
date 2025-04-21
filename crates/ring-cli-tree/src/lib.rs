use std::collections::{BTreeSet, HashMap};
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::iter::FusedIterator;

/// Formats given data as a tree
///
/// # Examples
///
/// ```
/// use ring_cli_tree::Tree;
///
/// let mut tree = Tree::new();
/// tree.add_root("a");
/// tree.add_node("aa", "a");
/// tree.add_node("aaa", "aa");
/// tree.add_root("b");
///
/// print!("{tree}");
/// ```
#[derive(Clone, Debug)]
pub struct Tree<K: Copy + Ord + Eq + Hash> {
    roots: BTreeSet<K>,
    parents: HashMap<K, K>,
    children: HashMap<K, BTreeSet<K>>,
}

impl<K: Copy + Ord + Eq + Hash> Tree<K> {
    /// Creates a new empty tree
    #[inline]
    pub fn new() -> Self {
        Tree {
            roots: BTreeSet::new(),
            parents: HashMap::new(),
            children: HashMap::new(),
        }
    }

    /// Add a new root to the tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_cli_tree::Tree;
    ///
    /// let mut tree = Tree::new();
    /// tree.add_root("a");
    ///
    /// assert_eq!(tree.len(), 1);
    /// ```
    pub fn add_root(&mut self, key: K) {
        if let Some(parent) = self.parents.get(&key) {
            if let Some(children) = self.children.get_mut(parent) {
                children.remove(&key);
            }

            self.parents.remove(&key);
        }

        self.roots.insert(key);
    }

    /// Add a new node to the tree.
    /// If the parent is not yet in the tree, it will be added a root.
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_cli_tree::Tree;
    ///
    /// let mut tree = Tree::new();
    /// tree.add_node("aa", "a");
    ///
    /// assert_eq!(tree.len(), 2);
    /// ```
    pub fn add_node(&mut self, key: K, parent: K) {
        if !self.parents.contains_key(&parent) {
            self.roots.insert(parent);
        }

        self.roots.remove(&key);
        self.parents.insert(key, parent);
        self.children.entry(parent)
            .or_default().insert(key);
    }
    
    pub fn contains_node(&self, key: K) -> bool {
        self.roots.contains(&key) || self.parents.contains_key(&key)
    }

    /// Returns an iterator over tree line items
    #[inline]
    pub fn iter(&self) -> TreeIter<'_, K> {
        TreeIter::new(self)
    }

    /// Returns `true` when the tree is empty
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_cli_tree::Tree;
    ///
    /// let mut tree = Tree::new();
    /// assert!(tree.is_empty());
    ///
    /// tree.add_node("aa", "a");
    /// assert!(!tree.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.roots.is_empty()
    }

    /// Returns the number of nodes in the table
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_cli_tree::Tree;
    ///
    /// let mut tree = Tree::new();
    /// tree.add_root("a");
    /// tree.add_node("aa", "a");
    /// tree.add_node("aaa", "aa");
    /// tree.add_root("b");
    ///
    /// assert_eq!(tree.len(), 4);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.roots.len() + self.parents.len()
    }
}

impl<K: Copy + Ord + Eq + Hash> Default for Tree<K> {
    #[inline]
    fn default() -> Self {
        Tree::new()
    }
}

impl<K: Copy + Display + Ord + Eq + Hash> Display for Tree<K> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.iter()
            .try_for_each(|node| writeln!(f, "{node}"))
    }
}

impl<'a, K: Copy + Ord + Eq + Hash> IntoIterator for &'a Tree<K> {
    type Item = TreeNode<'a, K>;
    type IntoIter = TreeIter<'a, K>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[derive(Clone, Debug)]
pub struct TreeIter<'a, K: Copy + Ord + Eq + Hash> {
    tree: &'a Tree<K>,
    stack: Vec<(&'a K, Vec<bool>)>,
}

impl<'a, K: Copy + Ord + Eq + Hash> TreeIter<'a, K> {
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

impl<'a, K: Copy + Ord + Eq + Hash> Iterator for TreeIter<'a, K> {
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

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.tree.len(), Some(self.tree.len()))
    }
}

impl<K: Copy + Ord + Eq + Hash> FusedIterator for TreeIter<'_, K> {}

pub struct TreeNode<'a, K: Clone + Ord + Eq + Hash> {
    pub key: &'a K,
    branches: Vec<bool>,
}

impl<K: Copy + Display + Ord + Eq + Hash> Display for TreeNode<'_, K> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_print_a_complex_tree() {
        let mut tree = Tree::new();
        tree.add_root("a");
        tree.add_node("aa", "a");
        tree.add_node("aaa", "aa");
        tree.add_node("aba", "ab");
        tree.add_node("ab", "a");
        tree.add_root("b");

        assert_eq!(tree.to_string(), concat!(
            "\u{25CF} a\n",
            "\u{251C}\u{2574}aa\n",
            "\u{2502} \u{2514}\u{2574}aaa\n",
            "\u{2514}\u{2574}ab\n",
            "  \u{2514}\u{2574}aba\n",
            "\u{25CF} b\n"
        ));
    }
}