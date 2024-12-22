use ring_color::ColorLabel;
use ring_tag::Tag;

/// Javascript package managers
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub enum PackageManager {
    #[default]
    Npm,
    Pnpm,
    Yarn
}

pub const PACKAGE_MANAGERS: [PackageManager; 3] = [PackageManager::Npm, PackageManager::Pnpm, PackageManager::Yarn];

impl PackageManager {
    /// Returns name of lockfile for this package manager
    pub fn lockfile(&self) -> &'static str {
        match self {
            PackageManager::Npm => "package-lock.json",
            PackageManager::Pnpm => "pnpm-lock.yaml",
            PackageManager::Yarn => "yarn.lock",
        }
    }

    /// Builds a tag matching this package manager
    pub fn tag(&self) -> Tag {
        match self {
            PackageManager::Npm => Tag::from("npm").with_color(ColorLabel::Red, (0xcc, 0x35, 0x34)),
            PackageManager::Pnpm => Tag::from("pnpm").with_color(ColorLabel::Yellow, (0xf9, 0xad, 0x00)),
            PackageManager::Yarn => Tag::from("yarn").with_color(ColorLabel::Blue, (0x2c, 0x8e, 0xbb)),
        }
    }
}
