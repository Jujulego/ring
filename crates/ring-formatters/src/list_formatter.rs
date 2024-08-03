use std::cmp::max;
use std::fmt::Display;
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

    pub fn display(&self) {
        for row in &self.rows {
            for (item, width) in zip(row, &self.widths) {
                let width = width + item.width() - display_width(item);
                print!("{item:width$} ");
            }

            println!();
        }
    }
}

impl<const N: usize> Default for ListFormatter<N> {
    fn default() -> Self {
        ListFormatter::new()
    }
}