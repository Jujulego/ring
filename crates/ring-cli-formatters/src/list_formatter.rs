use std::cmp::max;
use std::fmt::{Display, Formatter};
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

    pub fn add_row(&mut self, row: [&dyn Display; N]) {
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

            for (idx, item) in row.iter().enumerate() {
                if idx == N - 1 {
                    write!(f, "{item}")?;
                } else {
                    let width = self.widths[idx] + item.width() - display_width(item);
                    write!(f, "{item:width$} ")?;
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_print_data_in_align_columns() {
        let mut list = ListFormatter::new();
        
        list.add_row([&"test", &"title",      &"result"]);
        list.add_row([&"1",    &"first test", &"ok"]);
        list.add_row([&"42",   &"life",       &"ok"]);

        assert_eq!(format!("{list}"), [
            "test title      result",
            "1    first test ok",
            "42   life       ok",
        ].join("\n"));
    }
}