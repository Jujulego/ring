use ring_color::{ColorLabel, StableColor};
use ring_core::CodeLanguage;
use crate::constants::RUST_COLOR;

#[deprecated(note = "Please use `ring-rust::constants::RUST_LANGUAGE` instead")]
pub fn rust_language() -> CodeLanguage {
    CodeLanguage::new(
        "rust".to_string(),
        StableColor::new(ColorLabel::Red, RUST_COLOR),
    )
}