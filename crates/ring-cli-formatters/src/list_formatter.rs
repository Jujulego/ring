use std::cmp::max;
use std::fmt::{Display, Formatter};
use std::iter::zip;
use textwrap::core::display_width;
use unicode_width::UnicodeWidthStr;

#[derive(Debug)]
pub struct ListFormatter<const N: usize> {
    rows: Vec<[String; N]>,
    widths: [usize; N],
}

impl<const N: usize> ListFormatter<N> {
    pub fn new() -> ListFormatter<N> {
        ListFormatter { rows: Vec::new(), widths: [0; N] }
    }

    pub fn add_row<T: Display>(&mut self, row: [&T; N]) {
        let items = core::array::from_fn(|idx| format!("{}", row[idx]));

        self.widths.iter_mut()
            .zip(&items)
            .for_each(|(w, item)| *w = max(*w, display_width(item)));

        self.rows.push(items);
    }
}

impl<const N: usize> Default for ListFormatter<N> {
    fn default() -> Self {
        ListFormatter::new()
    }
}

impl<const N: usize> Display for ListFormatter<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (idx, row) in self.rows.iter().enumerate() {
            if idx != 0 {
                writeln!(f)?;
            }

            for (item, width) in zip(row, &self.widths) {
                let width = width + item.width() - display_width(item);
                write!(f, "{item:width$} ")?;
            }
        }
        
        Ok(())
    }
}