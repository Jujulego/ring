use std::cmp::max;
use std::fmt::{Display, Formatter};
use std::iter::FusedIterator;
use textwrap::core::display_width;

/// Formats given data as list with aligned values
///
/// # Examples
///
/// ```
/// use ring_cli_list::List;
///
/// let mut list = List::new();
/// list.push(vec!["Test".into(), "successful".into()]);
/// list.push(vec!["Test with a long name".into(), "successful".into()]);
/// 
/// print!("{list}");
/// ```
#[derive(Clone, Debug)]
pub struct List {
    items: Vec<Vec<String>>,
    widths: Vec<usize>,
}

impl List {
    /// Creates a new empty list
    #[inline]
    pub fn new() -> Self {
        List { items: Vec::new(), widths: Vec::new() }
    }
    
    /// Creates a new empty list, with given capacity
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        List { items: Vec::with_capacity(capacity), widths: Vec::new() }
    }
    
    /// Pushes a new value array to the end of the list
    pub fn push(&mut self, values: Vec<String>) {
        // Update values max widths
        self.widths.resize(values.len(), 0);

        values.iter()
            .zip(&mut self.widths)
            .for_each(|(values, width)| *width = max(*width, display_width(values)));

        // Store item
        self.items.push(values);
    }

    /// Returns an iterator over aligned list items
    #[inline]
    pub fn iter(&self) -> ListIter<'_> {
        ListIter {
            items: &self.items[..],
            widths: &self.widths,
        }
    }

    /// Returns `true` when the list is empty
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_cli_list::List;
    ///
    /// let mut list = List::new();
    /// assert!(list.is_empty());
    ///
    /// list.push(vec!["Test".into(), "successful".into()]);
    /// assert!(!list.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Returns the number of rows in the table
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_cli_list::List;
    ///
    /// let mut list = List::new();
    /// list.push(vec!["Test".into(), "successful".into()]);
    /// list.push(vec!["Test with a long name".into(), "successful".into()]);
    ///
    /// assert_eq!(list.len(), 2);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.items.len()
    }
}

impl Default for List {
    #[inline]
    fn default() -> Self {
        List::new()
    }
}

impl Display for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.iter()
            .try_for_each(|item| writeln!(f, "{item}"))
    }
}

impl Extend<Vec<String>> for List {
    fn extend<I: IntoIterator<Item = Vec<String>>>(&mut self, iter: I) {
        for item in iter {
            self.push(item);
        }
    }
}

impl FromIterator<Vec<String>> for List {
    fn from_iter<I: IntoIterator<Item = Vec<String>>>(items: I) -> Self {
        let iter = items.into_iter();
        
        let mut list = List::with_capacity(iter.size_hint().0);
        list.extend(iter);
        
        list
    }
}

impl<'a> IntoIterator for &'a List {
    type Item = ListItem<'a>;
    type IntoIter = ListIter<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ListIter<'a> {
    items: &'a [Vec<String>],
    widths: &'a [usize],
}

impl<'a> Iterator for ListIter<'a> {
    type Item = ListItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.items.is_empty() {
            return None;
        }

        let row = ListItem {
            values: &self.items[0],
            widths: self.widths,
        };

        self.items = &self.items[1..];

        Some(row)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.items.len(), Some(self.items.len()))
    }
}

impl DoubleEndedIterator for ListIter<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.items.is_empty() {
            return None;
        }

        let last_idx = self.items.len() - 1;

        let row = ListItem {
            values: &self.items[last_idx],
            widths: self.widths,
        };

        self.items = &self.items[..last_idx];

        Some(row)
    }
}

impl ExactSizeIterator for ListIter<'_> {}
impl FusedIterator for ListIter<'_> {}

#[derive(Clone, Copy, Debug)]
pub struct ListItem<'a> {
    values: &'a [String],
    widths: &'a [usize],
}

impl ListItem<'_> {
    pub fn get(&self, index: usize) -> Option<&'_ String> {
        self.values.get(index)
    }
}

impl Display for ListItem<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (i, value) in self.values.iter().enumerate() {
            write!(f, "{value}")?;

            if i < self.values.len() - 1 {
                write!(f, "{}", &" ".repeat(self.widths[i] - display_width(value) + 1))?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crossterm::style::Stylize;
    use super::*;

    #[test]
    fn it_should_print_rows_with_aligned_columns() {
        let mut list = List::new();
        list.push(vec!["Test".into(), "successful".into()]);
        list.push(vec!["Test with a long name".into(), "successful".into()]);

        let mut it = list.iter();
        assert_eq!(format!("{}", it.next().unwrap()), "Test                  successful");
        assert_eq!(format!("{}", it.next().unwrap()), "Test with a long name successful");
        assert!(it.next().is_none());
    }

    #[test]
    fn it_should_print_rows_in_reverse_order() {
        let mut list = List::new();
        list.push(vec!["Test".into(), "successful".into()]);
        list.push(vec!["Test with a long name".into(), "successful".into()]);

        let mut it = list.iter();
        assert_eq!(format!("{}", it.next_back().unwrap()), "Test with a long name successful");
        assert_eq!(format!("{}", it.next_back().unwrap()), "Test                  successful");
        assert!(it.next_back().is_none());
    }

    #[test]
    fn it_should_print_colored_rows_with_aligned_columns() {
        let mut list = List::new();
        list.push(vec!["Test".red().to_string(), "successful".stylize().to_string()]);
        list.push(vec!["Test with a long name".into(), "successful".into()]);

        let mut it = list.iter();
        assert_eq!(format!("{}", it.next().unwrap()), "\x1b[38;5;9mTest\x1b[39m                  successful");
        assert_eq!(format!("{}", it.next().unwrap()), "Test with a long name successful");
        assert!(it.next().is_none());
    }
}