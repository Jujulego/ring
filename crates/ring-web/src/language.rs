use rgb::Rgb;
use ring_color::{ColorLabel, StableColor};
use ring_core::CodeLanguage;
use ring_tag::Tag;
use std::string::ToString;

/// Enum of supported web languages
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WebLanguage {
    JavaScript,
    TypeScript,
}

impl From<WebLanguage> for CodeLanguage {
    fn from(value: WebLanguage) -> Self {
        match value {
            WebLanguage::JavaScript => CodeLanguage::new(
                "js".to_string(),
                StableColor::new(ColorLabel::Yellow, Rgb { r: 0xf7, g: 0xdf, b: 0x1e })
            ),
            WebLanguage::TypeScript => CodeLanguage::new(
                "ts".to_string(),
                StableColor::new(ColorLabel::Blue, Rgb { r: 0x00, g: 0x7a, b: 0xcc })
            ),
        }
    }
}

impl From<WebLanguage> for Tag {
    fn from(value: WebLanguage) -> Self {
        CodeLanguage::from(value).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_create_a_js_tag() {
        assert_eq!(Tag::from(WebLanguage::JavaScript).label(), "js");
    }

    #[test]
    fn it_should_create_a_ts_tag() {
        assert_eq!(Tag::from(WebLanguage::TypeScript).label(), "ts");
    }
}