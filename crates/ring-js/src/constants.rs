use crate::PackageManager;
use owo_colors::AnsiColors;
use owo_colors::DynColors::Ansi;
use ring_utils::Tag;

pub const MANIFEST: &str = "package.json";
pub const PACKAGE_MANAGERS: [PackageManager; 3] = [
    PackageManager::NPM,
    PackageManager::PNPM,
    PackageManager::Yarn,
];

pub fn js_tag() -> Tag {
    Tag::from("js").with_color(Ansi(AnsiColors::Yellow))
}

pub fn dev_tag() -> Tag {
    Tag::from("dev").with_color(Ansi(AnsiColors::Blue))
}

pub fn optional_tag() -> Tag {
    Tag::from("optional").with_color(Ansi(AnsiColors::Magenta))
}
