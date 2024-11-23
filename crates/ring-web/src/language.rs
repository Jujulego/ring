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