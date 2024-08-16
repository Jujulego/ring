use owo_colors::AnsiColors;
use owo_colors::DynColors::Ansi;
use ring_utils::Tag;
use crate::PackageManager;

pub const MANIFEST: &str = "package.json";
pub const LOCKFILES: [(PackageManager, &str); 2] = [
    (PackageManager::NPM, "package-lock.json"),
    (PackageManager::Yarn, "yarn.lock")
];

pub const JS_TAG: Tag = Tag::with_color("js", Ansi(AnsiColors::Yellow));