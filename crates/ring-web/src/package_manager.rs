use ring_color::ColorLabel;
use ring_tag::Tag;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub enum PackageManager {
    #[default]
    NPM,
    PNPM,
    Yarn
}

pub const PACKAGE_MANAGERS: [PackageManager; 3] = [PackageManager::NPM, PackageManager::PNPM, PackageManager::Yarn];

impl PackageManager {
    pub fn lockfile(&self) -> &'static str {
        match self {
            PackageManager::NPM => "package-lock.json",
            PackageManager::PNPM => "pnpm-lock.yaml",
            PackageManager::Yarn => "yarn.lock",
        }
    }

    pub fn tag(&self) -> Tag {
        match self {
            PackageManager::NPM => Tag::from("npm").with_color(ColorLabel::Red, (0xcc, 0x35, 0x34)),
            PackageManager::PNPM => Tag::from("pnpm").with_color(ColorLabel::Yellow, (0xf9, 0xad, 0x00)),
            PackageManager::Yarn => Tag::from("yarn").with_color(ColorLabel::Blue, (0x2c, 0x8e, 0xbb)),
        }
    }
}
