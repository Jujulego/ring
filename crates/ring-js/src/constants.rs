use owo_colors::AnsiColors;
use owo_colors::DynColors::Ansi;
use ring_utils::Tag;
use crate::PackageManager;

pub const MANIFEST: &str = "package.json";
pub const PACKAGE_MANAGERS: [PackageManager; 3] = [
    PackageManager::NPM,
    PackageManager::PNPM,
    PackageManager::Yarn,
];

pub const JS_TAG: Tag = Tag::with_color("js", Ansi(AnsiColors::Yellow));