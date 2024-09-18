use owo_colors::DynColors::Rgb;
use ring_utils::Tag;

pub const MANIFEST: &str = "Cargo.toml";

pub fn rust_tag() -> Tag {
    Tag::from("rust").with_color(Rgb(227, 59, 38))
}
