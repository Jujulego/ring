use std::fmt::{Display, Formatter};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub enum PackageManager {
    #[default]
    NPM,
    PNPM,
    Yarn
}

impl PackageManager {
    pub fn lockfile(&self) -> &'static str {
        match self {
            PackageManager::NPM => "package-lock.json",
            PackageManager::PNPM => "pnpm-lock.yaml",
            PackageManager::Yarn => "yarn.lock",
        }
    }
}

impl Display for PackageManager {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PackageManager::NPM => write!(f, "npm"),
            PackageManager::PNPM => write!(f, "pnpm"),
            PackageManager::Yarn => write!(f, "yarn")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_return_lockfile_name() {
        assert_eq!(PackageManager::NPM.lockfile(), "package-lock.json");
        assert_eq!(PackageManager::PNPM.lockfile(), "pnpm-lock.yaml");
        assert_eq!(PackageManager::Yarn.lockfile(), "yarn.lock");
    }

    #[test]
    fn it_should_display_package_manager_name() {
        assert_eq!(format!("{}", PackageManager::NPM), "npm");
        assert_eq!(format!("{}", PackageManager::PNPM), "pnpm");
        assert_eq!(format!("{}", PackageManager::Yarn), "yarn");
    }
}