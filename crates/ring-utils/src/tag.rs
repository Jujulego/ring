use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use owo_colors::{OwoColorize, Style};

#[derive(Debug)]
pub struct Tag {
    label: String,
    style: Style,
}

impl Tag {
    pub fn new(label: String) -> Tag {
        Tag { label, style: Style::new() }
    }

    pub fn with_style(label: String, style: Style) -> Tag {
        Tag { label, style }
    }

    pub fn label(&self) -> &String {
        &self.label
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label.style(self.style))
    }
}

impl Eq for Tag {}

impl PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        self.label.eq(&other.label)
    }
}

impl Ord for Tag {
    fn cmp(&self, other: &Self) -> Ordering {
        self.label.cmp(&other.label)
    }
}

impl PartialOrd for Tag {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
