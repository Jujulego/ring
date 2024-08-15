use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use owo_colors::{AnsiColors, DynColors, OwoColorize};
use owo_colors::DynColors::Ansi;

#[derive(Debug)]
pub struct Tag {
    label: &'static str,
    color: DynColors,
}

impl Tag {
    pub const fn new(label: &'static str) -> Tag {
        Tag { label, color: Ansi(AnsiColors::Default) }
    }

    pub const fn with_color(label: &'static str, color: DynColors) -> Tag {
        Tag { label, color }
    }

    pub const fn label(&self) -> &'static str {
        self.label
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label.color(self.color))
    }
}

impl Eq for Tag {}

impl PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        self.label.eq(other.label)
    }
}

impl Ord for Tag {
    fn cmp(&self, other: &Self) -> Ordering {
        self.label.cmp(other.label)
    }
}

impl PartialOrd for Tag {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
