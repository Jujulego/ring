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

#[cfg(test)]
mod tests {
    use mockall::mock;
    use mockall::predicate::eq;
    use super::*;

    mock!(
        Reader {}
        impl Read for Reader {
            fn read(&mut self, buf: &mut[u8]) -> std::io::Result<usize>;
            fn read_to_string(&mut self, buf: &mut String) -> std::io::Result<usize>;
        }
    );

    mock!(
        TestManifest {}
        impl Manifest for TestManifest {
            fn from_str(content: &str) -> anyhow::Result<Self>;
        }
    );

    #[test]
    fn it_should_use_read_to_string_then_call_from_str() {
        let mut reader = MockReader::new();
        reader.expect_read_to_string()
            .returning(|buffer| {
                buffer.push_str("test");
                Ok(4)
            });

        let ctx = MockTestManifest::from_str_context();
        ctx.expect()
            .with(eq("test"))
            .returning(|_| Ok(MockTestManifest::new()));

        assert!(MockTestManifest::from_reader(&mut reader).is_ok());
    }
}