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
    pub fn tag(&self) -> Tag {
        match self {
            WebLanguage::JavaScript => Tag::from("js").with_color((0xf7, 0xdf, 0x1e)),
            WebLanguage::TypeScript => Tag::from("ts").with_color((0x00, 0x7a, 0xcc)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_create_a_js_tag() {
        assert_eq!(WebLanguage::JavaScript.tag().label(), "js");
    }

    #[test]
    fn it_should_create_a_ts_tag() {
        assert_eq!(WebLanguage::TypeScript.tag().label(), "ts");
    }
}