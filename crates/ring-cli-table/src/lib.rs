use std::cmp::{max, Ordering};
use std::fmt::{Display, Formatter};
use std::iter::FusedIterator;
use crossterm::style::ContentStyle;
use textwrap::core::display_width;

////////////////////////////////////////////////////////////////////////////////
// Cli Table
////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
struct TableRow<const N: usize> {
    items: [String; N],
    style: ContentStyle,
}

/// Formats given data as table with aligned columns
///
/// # Examples
///
/// ```
/// use ring_cli_table::CliTable;
///
/// let mut table = CliTable::new();
/// table.add_row([&"Test", &"successful"]);
/// table.add_row([&"Test with a long name", &"successful"]);
///
/// for row in &table {
///   println!("{row}");
/// }
/// ```
#[derive(Clone, Debug)]
pub struct CliTable<const N: usize> {
    rows: Vec<TableRow<N>>,
    widths: [usize; N],
}

impl<const N: usize> CliTable<N> {
    /// Creates an empty CLiTable
    pub fn new() -> Self {
        CliTable {
            rows: Vec::new(),
            widths: [0; N],
        }
    }

    /// Adds given row to the table, and updates columns width
    #[inline]
    pub fn add_row(&mut self, row: [&dyn Display; N]) {
        self.add_styled_row(row, Default::default())
    }

    /// Adds given row to the table, and updates columns width
    pub fn add_styled_row(&mut self, row: [&dyn Display; N], style: ContentStyle) {
        let items = row.map(|d| format!("{}", d));

        self.widths.iter_mut()
            .zip(&items)
            .for_each(|(w, item)| *w = max(*w, display_width(item)));

        self.rows.push(TableRow { items, style });
    }

    /// Returns an iterator over table rows
    pub fn iter(&self) -> CliTableIter<'_, N> {
        CliTableIter {
            rows: &self.rows[..],
            widths: &self.widths,
        }
    }

    /// Returns `true` when the table is empty
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_cli_table::CliTable;
    ///
    /// let mut table = CliTable::new();
    /// assert!(table.is_empty());
    ///
    /// table.add_row([&"Test", &"successful"]);
    /// assert!(!table.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }


    /// Returns the number of rows in the table
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_cli_table::CliTable;
    ///
    /// let mut table = CliTable::new();
    /// table.add_row([&"Test", &"successful"]);
    /// table.add_row([&"Test with a long name", &"successful"]);
    ///
    /// assert_eq!(table.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn sort_by<F>(&mut self, mut compare: F)
    where
        F: FnMut(CliTableRow<N>, CliTableRow<N>) -> Ordering
    {
        self.rows.sort_by(|a, b| compare(
            CliTableRow { row: a, widths: &self.widths },
            CliTableRow { row: b, widths: &self.widths },
        ))
    }
}

impl<const N: usize> Default for CliTable<N> {
    #[inline]
    fn default() -> Self {
        CliTable::new()
    }
}

impl<'a, const N: usize> IntoIterator for &'a CliTable<N> {
    type Item = CliTableRow<'a, N>;
    type IntoIter = CliTableIter<'a, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

////////////////////////////////////////////////////////////////////////////////
// Cli Table Iterator
////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug)]
pub struct CliTableIter<'a, const N: usize> {
    rows: &'a [TableRow<N>],
    widths: &'a [usize; N],
}

impl<'a, const N: usize> Iterator for CliTableIter<'a, N> {
    type Item = CliTableRow<'a, N>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.rows.is_empty() {
            let row = CliTableRow {
                row: &self.rows[0],
                widths: self.widths,
            };

            self.rows = &self.rows[1..];
            Some(row)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.rows.len(), Some(self.rows.len()))
    }
}

impl<const N: usize> DoubleEndedIterator for CliTableIter<'_, N> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if !self.rows.is_empty() {
            let last_idx = self.rows.len() - 1;

            let row = CliTableRow {
                row: &self.rows[last_idx],
                widths: self.widths,
            };

            self.rows = &self.rows[..last_idx];
            Some(row)
        } else {
            None
        }
    }
}

impl<const N: usize> ExactSizeIterator for CliTableIter<'_, N> {}
impl<const N: usize> FusedIterator for CliTableIter<'_, N> {}

////////////////////////////////////////////////////////////////////////////////
// Cli Table Row
////////////////////////////////////////////////////////////////////////////////

/// Reference over a CliTable's row
#[derive(Clone, Copy, Debug)]
pub struct CliTableRow<'a, const N: usize> {
    row: &'a TableRow<N>,
    widths: &'a [usize; N],
}

impl<'a, const N: usize> CliTableRow<'a, N> {
    pub fn get(&self, idx: usize) -> Option<&'a String> {
        self.row.items.get(idx)
    }
}

impl<const N: usize> Display for CliTableRow<'_, N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut line = String::new();
        
        for (idx, item) in self.row.items.iter().enumerate() {
            line += item;
            
            if idx < N - 1 {
                line += &" ".repeat(self.widths[idx] - display_width(item) + 1);
            }
        }
        
        write!(f, "{}", self.row.style.apply(line))
    }
}

#[cfg(test)]
mod tests {
    use crossterm::style::Stylize;
    use super::*;

    #[test]
    fn it_should_print_rows_with_aligned_columns() {
        let mut table = CliTable::new();
        table.add_row([&"Test", &"successful"]);
        table.add_row([&"Test with a long name", &"successful"]);

        let mut it = table.iter();
        assert_eq!(format!("{}", it.next().unwrap()), "Test                  successful");
        assert_eq!(format!("{}", it.next().unwrap()), "Test with a long name successful");
        assert!(it.next().is_none());
    }

    #[test]
    fn it_should_print_rows_in_reverse_order() {
        let mut table = CliTable::new();
        table.add_row([&"Test", &"successful"]);
        table.add_row([&"Test with a long name", &"successful"]);

        let mut it = table.iter();
        assert_eq!(format!("{}", it.next_back().unwrap()), "Test with a long name successful");
        assert_eq!(format!("{}", it.next_back().unwrap()), "Test                  successful");
        assert!(it.next_back().is_none());
    }

    #[test]
    fn it_should_print_colored_rows_with_aligned_columns() {
        let mut table = CliTable::new();
        table.add_row([&"Test".red(), &"successful"]);
        table.add_row([&"Test with a long name", &"successful"]);

        let mut it = table.iter();
        assert_eq!(format!("{}", it.next().unwrap()), "\x1b[38;5;9mTest\x1b[39m                  successful");
        assert_eq!(format!("{}", it.next().unwrap()), "Test with a long name successful");
        assert!(it.next().is_none());
    }
}