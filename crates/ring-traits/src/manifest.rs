use std::io::Read;
use anyhow::Context;

pub trait Manifest : Sized {
    fn from_str(content: &str) -> anyhow::Result<Self>;
    
    fn from_reader<R: Read>(reader: &mut R) -> anyhow::Result<Self> {
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer).context("Unable to read")?;
        
        Self::from_str(&buffer)
    }
}