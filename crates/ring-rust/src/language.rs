use rgb::Rgb;
use ring_color::{ColorLabel, StableColor};
use ring_core::CodeLanguage;

pub fn rust_language() -> CodeLanguage {
    CodeLanguage::new(
        "rust".to_string(),
        StableColor::new(ColorLabel::BrightRed, Rgb { r: 0xe3, g: 0x3b, b: 0x26 }),
    )
}