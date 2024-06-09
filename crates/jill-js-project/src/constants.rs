use crate::PackageManager;

pub const MANIFEST: &str = "package.json";
pub const LOCKFILES: [(PackageManager, &str); 2] = [
    (PackageManager::NPM, "package-lock.json"),
    (PackageManager::Yarn, "yarn.lock")
];
