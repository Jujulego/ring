use rgb::Rgb;
use ring_core::units::Language;

pub const RUST_COLOR: Rgb<u8> = Rgb { r: 0xe3, g: 0x3b, b: 0x26 };
pub const RUST_LANGUAGE: Language = Language::new("rust").with_color(RUST_COLOR);