#[cfg(test)]
mod mock_supports_color;

use rgb::Rgb;
use std::cmp::Ordering;
use std::convert::Infallible;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::str::FromStr;

#[cfg(not(test))]
use supports_color::on as supports_color_on;

#[cfg(test)]
use mock_supports_color::on as supports_color_on;

#[cfg(feature = "owo")]
use owo_colors::{AnsiColors, Style, Styled};

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
/// assert_eq!(tag.scope(), None);
/// assert_eq!(tag.color(), None);
///
/// let tag = Tag::from("example").with_color((0, 255, 0));
/// assert_eq!(tag.label(), "example");
/// assert_eq!(tag.scope(), None);
/// assert_eq!(tag.color(), Some(&Rgb { r: 0, g: 255, b: 0 }));
///
/// let tag = Tag::from("example").with_scope("foo").with_color((0, 255, 0));
/// assert_eq!(tag.label(), "example");
/// assert_eq!(tag.scope(), Some("foo"));
/// assert_eq!(tag.color(), Some(&Rgb { r: 0, g: 255, b: 0 }));
///
/// ```
#[derive(Clone, Debug)]
pub struct Tag {
    label: String,
    scope: Option<String>,
    color: Option<Rgb<u8>>,
    #[cfg(feature = "owo")]
    ansi_color: Option<AnsiColors>,
}

impl Tag {
    /// Creates a new tag with given label
    pub fn new(label: String) -> Tag {
        Tag {
            label,
            scope: None,
            color: None,
            #[cfg(feature = "owo")]
            ansi_color: None,
        }
    }

    /// Adds given color to tag
    #[inline]
    pub fn with_color<C: Into<Rgb<u8>>>(self, color: C) -> Tag {
        self._with_color(color.into())
    }

    fn _with_color(self, color: Rgb<u8>) -> Tag {
        Tag { color: Some(color), ..self }
    }

    /// Adds given ansi color to tag (for basic terminal support)
    #[cfg(feature = "owo")]
    pub fn with_ansi_color(self, ansi_colors: AnsiColors) -> Tag {
        Tag { ansi_color: Some(ansi_colors), ..self }
    }

    /// Adds given scope to tag
    #[inline]
    pub fn with_scope<S: ToString + ?Sized>(self, scope: &S) -> Tag {
        self._with_scope(scope.to_string())
    }

    fn _with_scope(self, scope: String) -> Tag {
        Tag { scope: Some(scope), ..self }
    }

    /// Displays colored tag using owo-colors
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use owo_colors::{OwoColorize, Style};
    /// use ring_tag::Tag;
    ///
    /// assert_eq!(
    ///     format!("{}", Tag::from("hello").with_scope("a").with_color((255, 0, 0)).styled()),
    ///     format!("{}", "a:hello".style(Style::new().color(owo_colors::Rgb(255, 0, 0))))
    /// );
    /// ```
    #[cfg(feature = "owo")]
    pub fn styled(&self) -> Styled<&Tag> {
        self.styled_for(supports_color::Stream::Stdout)
    }

    /// Displays colored tag using owo-colors, according to supported colors of given stream
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use owo_colors::{OwoColorize, Style};
    /// use supports_color::Stream;
    /// use ring_tag::Tag;
    ///
    /// assert_eq!(
    ///     format!("{}", Tag::from("hello").with_scope("a").with_color((255, 0, 0)).styled_for(Stream::Stdout)),
    ///     format!("{}", "a:hello".style(Style::new().color(owo_colors::Rgb(255, 0, 0))))
    /// );
    /// ```
    #[cfg(feature = "owo")]
    pub fn styled_for(&self, stream: supports_color::Stream) -> Styled<&Tag> {
        let mut style = Style::new();

        if let Some(support) = dbg!(supports_color_on(stream)) {
            if support.has_basic {
                if let Some(ansi_color) = self.ansi_color {
                    style = style.color(ansi_color);
                }
            }

            if support.has_16m {
                if let Some(color) = self.color {
                    style = style.color(owo_colors::Rgb(color.r, color.g, color.b));
                }
            }
        }

        style.style(self)
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn color(&self) -> Option<&Rgb<u8>> {
        self.color.as_ref()
    }

    pub fn scope(&self) -> Option<&str> {
        self.scope.as_deref()
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

impl Hash for Tag {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.scope.hash(state);
        self.label.hash(state);
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
    fn tags(&self) -> Vec<Tag> {
        vec![]
    }
}

////////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::mock;

    #[cfg(feature = "owo")]
    use owo_colors::OwoColorize;

    macro_rules! hash {
        ($v:expr) => {{
            let mut hasher = std::hash::DefaultHasher::new();
            $v.hash(&mut hasher);
            hasher.finish()
        }};
    }

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
    fn it_should_have_same_hash_with_same_label_and_scope() {
        assert_eq!(
            hash!(Tag::from("hello")),
            hash!(Tag::from("hello").with_color((0, 0, 0)))
        );
        assert_ne!(
            hash!(Tag::from("hello")),
            hash!(Tag::from("hello").with_scope("a"))
        );
        assert_ne!(
            hash!(Tag::from("hello").with_scope("a")),
            hash!(Tag::from("hello").with_scope("b"))
        );
        assert_eq!(
            hash!(Tag::from("hello").with_scope("a")),
            hash!(Tag::from("hello").with_scope("a").with_color((0, 0, 0)))
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
    fn it_should_convert_from_string() {
        assert_eq!(
            Tag::from("a:hello"),
            Tag::from("hello").with_scope("a")
        );
        assert_eq!(
            Tag::from_str("a:hello"),
            Ok(Tag::from("hello").with_scope("a"))
        );
    }
    
    #[test]
    fn it_should_print_uncolored_label() {
        assert_eq!(format!("{}", Tag::from("hello")), "hello");
        assert_eq!(format!("{}", Tag::from("hello").with_scope("a")), "a:hello");
    }

    mock! {
        TestStream {}
    }

    #[test]
    #[cfg(feature = "owo")]
    fn it_should_print_ansi_colored_label() {
        mock_supports_color::set_has_16m();

        assert_eq!(
            format!("{}", Tag::from("hello").with_ansi_color(AnsiColors::Red).styled()),
            format!("{}", "hello".style(Style::new().color(AnsiColors::Red)))
        );
        assert_eq!(
            format!("{}", Tag::from("hello").with_scope("a").with_ansi_color(AnsiColors::Red).styled()),
            format!("{}", "a:hello".style(Style::new().color(AnsiColors::Red)))
        );
    }

    #[test]
    #[cfg(feature = "owo")]
    fn it_should_print_rgb_colored_label() {
        mock_supports_color::set_has_16m();

        assert_eq!(
            format!("{}", Tag::from("hello").with_color((255, 0, 0)).styled()),
            format!("{}", "hello".style(Style::new().color(owo_colors::Rgb(255, 0, 0))))
        );
        assert_eq!(
            format!("{}", Tag::from("hello").with_scope("a").with_color((255, 0, 0)).styled()),
            format!("{}", "a:hello".style(Style::new().color(owo_colors::Rgb(255, 0, 0))))
        );
    }

    mock! {
        TestTagged {}
        impl Tagged for TestTagged {}
    }
    
    #[test]
    fn tagged_tags_should_return_an_empty_vector_by_default() {
        let tt = MockTestTagged::new();
        
        assert_eq!(tt.tags(), vec![]);
    }
}