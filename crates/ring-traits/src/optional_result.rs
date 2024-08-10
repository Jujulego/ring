#[derive(Debug, Eq, PartialEq)]
pub enum OptionalResult<T, E = anyhow::Error> {
    Found(T),
    Err(E),
    None,
}

impl<T, E> OptionalResult<T, E> {
    pub fn filter<F>(self, f: F) -> OptionalResult<T, E>
    where F: FnOnce(&T) -> bool
    {
        match self {
            OptionalResult::Found(val) if f(&val) => OptionalResult::Found(val),
            OptionalResult::Found(_) | OptionalResult::None => OptionalResult::None,
            OptionalResult::Err(err) => OptionalResult::Err(err),
        }
    }

    pub fn inspect<F>(self, f: F) -> OptionalResult<T, E>
    where F: FnOnce(&T)
    {
        if let OptionalResult::Found(val) = &self {
            f(val);
        }

        self
    }

    pub fn map<R, F>(self, f: F) -> OptionalResult<R, E>
    where F: FnOnce(T) -> R
    {
        match self {
            OptionalResult::Found(data) => OptionalResult::Found(f(data)),
            OptionalResult::Err(err) => OptionalResult::Err(err),
            OptionalResult::None => OptionalResult::None
        }
    }
}

impl<T, E> From<Result<T, E>> for OptionalResult<T, E> {
    fn from(res: Result<T, E>) -> Self {
        match res {
            Ok(val) => OptionalResult::Found(val),
            Err(err) => OptionalResult::Err(err),
        }
    }
}

impl<T, E> From<Option<T>> for OptionalResult<T, E> {
    fn from(opt: Option<T>) -> Self {
        match opt {
            Some(val) => OptionalResult::Found(val),
            None => OptionalResult::None,
        }
    }
}

impl<T, E> OptionalResult<T, E> {
    pub fn into_option(self) -> Option<Result<T, E>> {
        match self {
            OptionalResult::Found(val) => Some(Ok(val)),
            OptionalResult::Err(err) => Some(Err(err)),
            OptionalResult::None => None,
        }
    }

    pub fn into_result(self) -> Result<Option<T>, E> {
        match self {
            OptionalResult::Found(val) => Ok(Some(val)),
            OptionalResult::Err(err) => Err(err),
            OptionalResult::None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::optional_result::OptionalResult;

    #[test]
    fn it_should_convert_result_into_detector_result() {
        assert_eq!(OptionalResult::<&str, ()>::from(Ok("test")), OptionalResult::Found("test"));
        assert_eq!(OptionalResult::<(), &str>::from(Err("test")), OptionalResult::Err("test"));
    }

    #[test]
    fn it_should_convert_option_into_detector_result() {
        assert_eq!(OptionalResult::<&str, ()>::from(Some("test")), OptionalResult::Found("test"));
        assert_eq!(OptionalResult::<(),   ()>::from(None), OptionalResult::None);
    }

    #[test]
    fn it_should_convert_detector_result_into_result() {
        assert_eq!(OptionalResult::<&str, ()>::Found("test").into_result(), Ok(Some("test")));
        assert_eq!(OptionalResult::<(), &str>::Err("test").into_result(), Err("test"));
        assert_eq!(OptionalResult::<(),   ()>::None.into_result(), Ok(None));
    }

    #[test]
    fn it_should_convert_detector_result_into_option() {
        assert_eq!(OptionalResult::<&str, ()>::Found("test").into_option(), Some(Ok("test")));
        assert_eq!(OptionalResult::<(), &str>::Err("test").into_option(), Some(Err("test")));
        assert_eq!(OptionalResult::<(),   ()>::None.into_option(), None);
    }
}