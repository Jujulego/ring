use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PackageManager {
    NPM,
    Yarn
}

impl Default for PackageManager {
    fn default() -> Self {
        PackageManager::NPM
    }
}

impl Display for PackageManager {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PackageManager::NPM => write!(f, "npm"),
            PackageManager::Yarn => write!(f, "yarn")
        }
    }
}