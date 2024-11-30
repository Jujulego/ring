use ring_color::ColorLabel;
use ring_tag::Tag;

////////////////////////////////////////////////////////////////////////////////
// Language
////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WebLanguage {
    JavaScript,
    TypeScript,
}

impl WebLanguage {
    pub fn tag(&self, label: String) -> Tag {
        match self {
            WebLanguage::JavaScript => Tag::new(label).with_scope("js").with_color(ColorLabel::Yellow, (0xf7, 0xdf, 0x1e)),
            WebLanguage::TypeScript => Tag::new(label).with_scope("ts").with_color(ColorLabel::Blue, (0x00, 0x7a, 0xcc)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_create_a_js_tag() {
        assert_eq!(WebLanguage::JavaScript.tag("test".to_string()).scope(), Some("js"));
    }

    #[test]
    fn it_should_create_a_ts_tag() {
        assert_eq!(WebLanguage::TypeScript.tag("test".to_string()).scope(), Some("ts"));
    }
}