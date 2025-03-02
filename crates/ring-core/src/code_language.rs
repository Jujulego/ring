use ring_color::StableColor;
use ring_tag::Tag;

////////////////////////////////////////////////////////////////////////////////
// Code Language
////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct CodeLanguage {
    name: String,
    color: StableColor,
}

impl CodeLanguage {
    pub const fn new(name: String, color: StableColor) -> CodeLanguage {
        CodeLanguage { name, color }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn color(&self) -> &StableColor {
        &self.color
    }
}

impl From<CodeLanguage> for Tag {
    fn from(l: CodeLanguage) -> Tag {
        Tag::new(l.name).with_stable_color(l.color)
    }
}

impl From<&CodeLanguage> for Tag {
    fn from(l: &CodeLanguage) -> Tag {
        Tag::new(l.name.clone()).with_stable_color(l.color)
    }
}

