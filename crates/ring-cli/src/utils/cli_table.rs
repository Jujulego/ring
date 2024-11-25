use std::cmp::max;
use std::fmt::{Display, Formatter};
use std::iter::FusedIterator;
use textwrap::core::display_width;
use unicode_width::UnicodeWidthStr;

////////////////////////////////////////////////////////////////////////////////
// Cli Table
////////////////////////////////////////////////////////////////////////////////

/// Formats given data as table with aligned columns
#[derive(Clone, Debug)]
pub struct CliTable<const N: usize> {
    rows: Vec<[String; N]>,
    widths: [usize; N],
}

impl<const N: usize> CliTable<N> {
    pub fn new() -> Self {
        CliTable {
            rows: Vec::new(),
            widths: [0; N],
        }
    }

    pub fn add_row(&mut self, row: [&dyn Display; N]) {
        let items = core::array::from_fn(|idx| format!("{}", row[idx]));

        self.widths.iter_mut()
            .zip(&items)
            .for_each(|(w, item)| *w = max(*w, display_width(item)));

        self.rows.push(items);
    }

    pub fn iter(&self) -> CliTableIter<'_, N> {
        CliTableIter {
            rows: &self.rows[..],
            widths: &self.widths,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn len(&self) -> usize {
        self.rows.len()
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
    rows: &'a [[String; N]],
    widths: &'a [usize; N],
}

impl<'a, const N: usize> Iterator for CliTableIter<'a, N> {
    type Item = CliTableRow<'a, N>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.rows.is_empty() {
            let row = CliTableRow {
                items: &self.rows[0],
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

impl<'a, const N: usize> DoubleEndedIterator for CliTableIter<'a, N> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if !self.rows.is_empty() {
            let last_idx = self.rows.len() - 1;
            
            let row = CliTableRow {
                items: &self.rows[last_idx],
                widths: self.widths,
            };

            self.rows = &self.rows[..last_idx];
            Some(row)
        } else {
            None
        }
    }
}

impl<'a, const N: usize> ExactSizeIterator for CliTableIter<'a, N> {}
impl<'a, const N: usize> FusedIterator for CliTableIter<'a, N> {}

////////////////////////////////////////////////////////////////////////////////
// Cli Table Row
////////////////////////////////////////////////////////////////////////////////

/// Reference over a CliTable's row
#[derive(Clone, Copy, Debug)]
pub struct CliTableRow<'a, const N: usize> {
    items: &'a [String; N],
    widths: &'a [usize; N],
}

impl<'a, const N: usize> Display for CliTableRow<'a, N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (idx, item) in self.items.iter().enumerate() {
            if idx < N {
                let width = self.widths[idx] + item.width() - display_width(item);
                write!(f, "{item:width$} ")?;
            } else {
                write!(f, "{item}")?;
            }
        }

        Ok(())
    }
}
