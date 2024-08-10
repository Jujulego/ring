use crate::OptionalResult::{Empty, Fail, Found};

#[derive(Debug, Eq, PartialEq)]
pub enum OptionalResult<T, E = anyhow::Error> {
    Found(T),
    Fail(E),
    Empty,
}

impl<T, E> OptionalResult<T, E> {
    pub fn filter<F>(self, f: F) -> OptionalResult<T, E>
    where
        F: FnOnce(&T) -> bool,
    {
        match self {
            Found(val) if f(&val) => Found(val),
            Found(_) | Empty => Empty,
            Fail(err) => Fail(err),
        }
    }

    pub fn inspect<F>(self, f: F) -> OptionalResult<T, E>
    where
        F: FnOnce(&T),
    {
        if let Found(val) = &self {
            f(val);
        }

        self
    }

    pub fn map<R, F>(self, f: F) -> OptionalResult<R, E>
    where
        F: FnOnce(T) -> R,
    {
        match self {
            Found(data) => Found(f(data)),
            Fail(err) => Fail(err),
            Empty => Empty,
        }
    }
}

impl<T, E> From<Result<T, E>> for OptionalResult<T, E> {
    fn from(res: Result<T, E>) -> Self {
        match res {
            Ok(val) => Found(val),
            Err(err) => Fail(err),
        }
    }
}

impl<T, E> From<Option<T>> for OptionalResult<T, E> {
    fn from(opt: Option<T>) -> Self {
        match opt {
            Some(val) => Found(val),
            None => Empty,
        }
    }
}

impl<T, E> OptionalResult<T, E> {
    pub fn into_option(self) -> Option<Result<T, E>> {
        match self {
            Found(val) => Some(Ok(val)),
            Fail(err) => Some(Err(err)),
            Empty => None,
        }
    }

    pub fn into_result(self) -> Result<Option<T>, E> {
        match self {
            Found(val) => Ok(Some(val)),
            Fail(err) => Err(err),
            Empty => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use mockall::mock;
    use mockall::predicate::eq;
    use crate::optional_result::OptionalResult;
    use crate::OptionalResult::{Empty, Fail, Found};

    type OR = OptionalResult<&'static str, &'static str>;
    
    #[test]
    fn it_should_convert_result_into_detector_result() {
        assert_eq!(OR::from(Ok("test")), Found("test"));
        assert_eq!(OR::from(Err("test")), Fail("test"));
    }

    #[test]
    fn it_should_convert_option_into_detector_result() {
        assert_eq!(OR::from(Some("test")), Found("test"));
        assert_eq!(OR::from(None), Empty);
    }

    #[test]
    fn it_should_convert_detector_result_into_result() {
        assert_eq!(OR::Found("test").into_result(), Ok(Some("test")));
        assert_eq!(OR::Fail("test").into_result(), Err("test"));
        assert_eq!(OR::Empty.into_result(), Ok(None));
    }

    #[test]
    fn it_should_convert_detector_result_into_option() {
        assert_eq!(OR::Found("test").into_option(), Some(Ok("test")));
        assert_eq!(OR::Fail("test").into_option(), Some(Err("test")));
        assert_eq!(OR::Empty.into_option(), None);
    }

    #[test]
    fn it_should_filter_optional_result() {
        assert_eq!(OR::Found("test").filter(|_| true), Found("test"));
        assert_eq!(OR::Found("test").filter(|_| false), Empty);

        assert_eq!(OR::Fail("test").filter(|_| true), Fail("test"));
        assert_eq!(OR::Fail("test").filter(|_| false), Fail("test"));

        assert_eq!(OR::Empty.filter(|_| true), Empty);
        assert_eq!(OR::Empty.filter(|_| false), Empty);
    }

    #[test]
    fn it_should_map_optional_result() {
        assert_eq!(OR::Found("test").map(|s| s.len()), Found(4));
        assert_eq!(OR::Fail("test").map(|s| s.len()), Fail("test"));
        assert_eq!(OR::Empty.map(|s| s.len()), Empty);
    }

    #[test]
    fn it_should_inspect_optional_result() {
        mock!(
            Inspector {
                fn view(&self, val: &str) -> ();
            }
        );
        
        let mut inspector = MockInspector::new();
        inspector.expect_view()
            .with(eq("test"))
            .times(1)
            .return_const(());
        
        assert_eq!(OR::Found("test").inspect(|&s| inspector.view(s)), Found("test"));
        
        inspector.checkpoint();
        
        assert_eq!(OR::Fail("test").inspect(|&s| inspector.view(s)), Fail("test"));
        assert_eq!(OR::Empty.inspect(|&s| inspector.view(s)), Empty);
    }
}
