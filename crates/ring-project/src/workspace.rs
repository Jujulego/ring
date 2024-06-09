use std::path::Path;

pub trait Workspace {
    fn name(&self) -> &str;
    fn root(&self) -> &Path;
    fn version(&self) -> Option<&str>;
    
    fn reference(&self) -> String {
        if let Some(version) = &self.version() {
            format!("{}@{version}", self.name())
        } else {
            self.name().to_string()
        }
    }
}
