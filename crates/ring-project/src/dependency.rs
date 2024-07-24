use std::fmt::{Display, Formatter};
use crate::requirement::Requirement;

#[derive(Debug)]
pub struct Dependency {
    target: String,
    requirement: Requirement,
}

impl Dependency {
    pub fn new(target: String, requirement: Requirement) -> Dependency {
        Dependency { target, requirement }
    }
    
    pub fn target(&self) -> &String {
        &self.target
    }

    pub fn requirement(&self) -> &Requirement {
        &self.requirement
    }
}

impl Display for Dependency {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}@{}", self.target, self.requirement)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use super::*;

    #[test]
    fn it_should_print_target_with_requirement() {
        let req = Requirement::PATH(PathBuf::from("/test"));
        let dependency = Dependency::new("test".to_string(), req);
        
        assert_eq!(format!("{dependency}"), "test@path:/test");
    }
}