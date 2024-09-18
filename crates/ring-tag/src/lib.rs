use rgb::Rgb;
use std::cmp::Ordering;
use std::convert::Infallible;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
#[cfg(feature = "owo")]
use owo_colors::{Style, Styled};

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
    scope: Option<String>,
}

impl Tag {
    /// Creates a new tag with given label
    pub fn new(label: String) -> Tag {
        Tag { label, scope: None, color: None }
    }

    /// Adds given color to tag
    #[inline]
    pub fn with_color<C: Into<Rgb<u8>>>(self, color: C) -> Tag {
        self._with_color(color.into())
    }

    fn _with_color(self, color: Rgb<u8>) -> Tag {
        Tag { color: Some(color), ..self }
    }

    /// Adds given scope to tag
    #[inline]
    pub fn with_scope<S: ToString + ?Sized>(self, scope: &S) -> Tag {
        self._with_scope(scope.to_string())
    }
    
    fn _with_scope(self, scope: String) -> Tag {
        Tag { scope: Some(scope), ..self }
    }

    #[cfg(feature = "owo")]
    pub fn styled(&self) -> Styled<&Tag> {
        let mut style = Style::new();

        if let Some(color) = self.color() {
            style = style.color(owo_colors::Rgb(color.r, color.g, color.b));
        }

        style.style(self)
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn color(&self) -> Option<&Rgb<u8>> {
        self.color.as_ref()
    }

    pub fn scope(&self) -> Option<&String> {
        self.scope.as_ref()
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(scope) = self.scope() {
            write!(f, "{}:{}", scope, self.label)
        } else {
            self.label.fmt(f)
        }
    }
}

impl From<&str> for Tag {
    fn from(value: &str) -> Self {
        if let Some((scope, label)) = value.split_once(":") {
            Tag::new(label.to_string()).with_scope(scope)
        } else {
            Tag::new(value.to_string())
        }
    }
}

impl FromStr for Tag {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Eq for Tag {}

impl PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        self.scope == other.scope && self.label == other.label
    }
}

impl Ord for Tag {
    fn cmp(&self, other: &Self) -> Ordering {
        self.scope.cmp(&other.scope).then(self.label.cmp(&other.label))
    }
}

impl PartialOrd for Tag {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

////////////////////////////////////////////////////////////////////////////////
// Tagged
////////////////////////////////////////////////////////////////////////////////

/// Object holding one or many tags
pub trait Tagged {
    fn tags(&self) -> Vec<Tag>;
}

////////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "owo")]
    use owo_colors::OwoColorize;

    #[test]
    fn it_should_be_equal_with_same_label_and_scope() {
        assert_eq!(
            Tag::from("hello"),
            Tag::from("hello").with_color((0, 0, 0))
        );
        assert_ne!(
            Tag::from("hello"),
            Tag::from("hello").with_scope("a")
        );
        assert_ne!(
            Tag::from("hello").with_scope("a"),
            Tag::from("hello").with_scope("b")
        );
        assert_eq!(
            Tag::from("hello").with_scope("a"),
            Tag::from("hello").with_scope("a").with_color((0, 0, 0))
        );
    }

    #[test]
    fn it_should_compare_by_scopes_then_by_labels() {
        assert_eq!(Tag::from("a").cmp(&Tag::from("b")), Ordering::Less);
        assert_eq!(Tag::from("hello").with_scope("a").cmp(&Tag::from("hello").with_scope("b")), Ordering::Less);
        
        assert_eq!(Tag::from("a").partial_cmp(&Tag::from("b")), Some(Ordering::Less));
        assert_eq!(Tag::from("hello").with_scope("a").partial_cmp(&Tag::from("hello").with_scope("b")), Some(Ordering::Less));
    }
    
    #[test]
    fn it_should_print_uncolored_label() {
        assert_eq!(format!("{}", Tag::from("hello")), "hello");
        assert_eq!(format!("{}", Tag::from("hello").with_scope("a")), "a:hello");
    }
    
    #[test]
    #[cfg(feature = "owo")]
    fn it_should_print_colored_label() {
        assert_eq!(
            format!("{}", Tag::from("hello").with_color((255, 0, 0)).styled()),
            format!("{}", "hello".style(Style::new().color(owo_colors::Rgb(255, 0, 0))))
        );
        assert_eq!(
            format!("{}", Tag::from("hello").with_scope("a").with_color((255, 0, 0)).styled()),
            format!("{}", "a:hello".style(Style::new().color(owo_colors::Rgb(255, 0, 0))))
        );
    }
}