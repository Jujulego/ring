use std::cmp::Ordering;
use rgb::Rgb;

////////////////////////////////////////////////////////////////////////////////
// Tag
////////////////////////////////////////////////////////////////////////////////

/// Possibly colored tag.
///
/// # Examples
///
/// ```
/// use rgb::Rgb;
/// use ring_tag::Tag;
///
/// let tag = Tag::from("example");
/// assert_eq!(tag.label(), "example");
/// assert_eq!(tag.color(), None);
///
/// let tag = Tag::from("example").with_color((0, 255, 0));
/// assert_eq!(tag.label(), "example");
/// assert_eq!(tag.color(), Some(&Rgb { r: 0, g: 255, b: 0}));
///
/// ```
#[derive(Debug)]
pub struct Tag {
    label: String,
    color: Option<Rgb<u8>>,
}

impl Tag {
    /// Creates a new tag with given label
    pub fn new(label: String) -> Tag {
        Tag { label, color: None }
    }

    /// Adds given color to tag
    pub fn with_color<C: Into<Rgb<u8>>>(self, color: C) -> Tag {
        Tag { label: self.label, color: Some(color.into()) }
    }

    fn _with_color(self, color: Rgb<u8>) -> Tag {
        Tag { label: self.label, color: Some(color) }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn color(&self) -> Option<&Rgb<u8>> {
        self.color.as_ref()
    }
}

impl From<&str> for Tag {
    fn from(value: &str) -> Self {
        Tag::new(value.to_string())
    }
}

impl Eq for Tag {}

impl PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}

impl Ord for Tag {
    fn cmp(&self, other: &Self) -> Ordering {
        self.label.cmp(&other.label)
    }
}

impl PartialOrd for Tag {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.label.cmp(&other.label))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_be_equal_with_same_label() {
        assert_eq!(
            Tag::from("hello"),
            Tag::from("hello").with_color((0, 0, 0))
        );
        assert_eq!(
            Tag::from("hello").with_color((255, 0, 0)),
            Tag::from("hello").with_color((0, 255, 0))
        );
    }

    #[test]
    fn it_should_compare_as_their_labels() {
        assert_eq!(
            Tag::from("a").cmp(&Tag::from("b").with_color((0, 0, 0))),
            "a".cmp("b")
        );
        assert_eq!(
            Tag::from("a").partial_cmp(&Tag::from("b").with_color((0, 0, 0))),
            "a".partial_cmp("b")
        );
    }
}