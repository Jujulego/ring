#[macro_export]
macro_rules! absolute_path {
    ($path:literal) => {
        std::path::PathBuf::from((if cfg!(windows) { r"C:\" } else { "/" }).to_owned() + $path)
    }
}
