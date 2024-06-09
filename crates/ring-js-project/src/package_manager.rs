use std::fmt::{Display, Formatter};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub enum PackageManager {
    #[default]
    NPM,
    Yarn
}

impl Display for PackageManager {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PackageManager::NPM => write!(f, "npm"),
            PackageManager::Yarn => write!(f, "yarn")
        }
    }
}