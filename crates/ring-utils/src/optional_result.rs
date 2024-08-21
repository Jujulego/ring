use crate::OptionalResult::{Empty, Fail, Found};

/// Combination of Option and Result
///
/// # Example
///
/// `OptionalResult<T, E>` is the same as `Option<Result<T, E>>`
/// ```
/// use ring_utils::OptionalResult::{self, *};
///
/// assert_eq!(Found::<&str, ()>("test"), Some(Ok("test")));
/// assert_eq!(Empty::<&str, ()>, None::<Result<_, _>>);
/// assert_eq!(Fail::<&str, &str>("failed"), Some(Err("failed")));
/// ```
///
/// `OptionalResult<T, E>` is the same as `Result<Option<T>, E>`
/// ```
/// use ring_utils::OptionalResult::{self, *};
///
/// assert_eq!(Found::<&str, ()>("test"), Ok(Some("test")));
/// assert_eq!(Empty::<&str, ()>, Ok(None));
/// assert_eq!(Fail::<&str, &str>("failed"), Err::<Option<_>, _>("failed"));
/// ```
#[derive(Debug, Eq)]
pub enum OptionalResult<T, E = anyhow::Error> {
    Found(T),
    Fail(E),
    Empty,
}

impl<T, E> OptionalResult<T, E> {
    /// Returns [`Empty`] if the optional result is [`Empty`], [`Fail`] if it is [`Fail`] otherwise
    /// calls `f` with the wrapped value and return the result.
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_utils::OptionalResult::{self, *};
    ///
    /// fn euclidean_divide(a: i32, b: i32) -> OptionalResult<i32, &'static str> {
    ///     match (a, b) {
    ///         (_, 0) => Fail("Cannot divide by 0"),
    ///         (a, b) if a % b == 0 => Found(a / b),
    ///         (_, _) => Empty,
    ///     }
    /// }
    ///
    /// assert_eq!(Found(4).and_then(|n| euclidean_divide(n, 2)), Found(2));
    /// assert_eq!(Found(4).and_then(|n| euclidean_divide(n, 3)), Empty);
    /// assert_eq!(Found(4).and_then(|n| euclidean_divide(n, 0)), Fail("Cannot divide by 0"));
    /// assert_eq!(Empty.and_then(|n| euclidean_divide(n, 2)), Empty);
    /// assert_eq!(Fail("early").and_then(|n| euclidean_divide(n, 2)), Fail("early"));
    /// ```
    pub fn and_then<R, F>(self, f: F) -> OptionalResult<R, E>
    where
        F: FnOnce(T) -> OptionalResult<R, E>,
    {
        match self {
            Found(val) => f(val),
            Fail(err) => Fail(err),
            Empty => Empty,
        }
    }

    /// Returns given value if the optional result is [`Empty`], otherwise it keeps the
    /// current value
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_utils::OptionalResult::{self, *};
    ///
    /// assert_eq!(Found::<i32, ()>(2).fail_or(42), Found(2));
    /// assert_eq!(Empty::<i32, ()>.fail_or(42), Found(42));
    /// assert_eq!(Fail::<i32, &str>("early").fail_or(42), Fail("early"));
    /// ```
    pub fn fail_or(self, val: T) -> OptionalResult<T, E> {
        if matches!(self, Empty) { Found(val) } else { self }
    }

    /// Returns default value if the optional result is [`Empty`], otherwise it keeps the
    /// current value
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_utils::OptionalResult::{self, *};
    ///
    /// assert_eq!(Found::<i32, ()>(2).fail_or_default(), Found(2));
    /// assert_eq!(Empty::<i32, ()>.fail_or_default(), Found(0));
    /// assert_eq!(Fail::<i32, &str>("early").fail_or_default(), Fail("early"));
    /// ```
    pub fn fail_or_default(self) -> OptionalResult<T, E>
    where
        T: Default
    {
        self.fail_or(T::default())
    }

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

impl<T, E> Default for OptionalResult<T, E> {
    fn default() -> Self {
        Empty
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

impl<T, E> From<OptionalResult<T, E>> for Result<Option<T>, E> {
    fn from(res: OptionalResult<T, E>) -> Self {
        match res {
            Found(val) => Ok(Some(val)),
            Fail(err) => Err(err),
            Empty => Ok(None),
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

impl<T, E> From<OptionalResult<T, E>> for Option<Result<T, E>> {
    fn from(res: OptionalResult<T, E>) -> Self {
        match res {
            Found(val) => Some(Ok(val)),
            Fail(err) => Some(Err(err)),
            Empty => None,
        }
    }
}

impl<T: PartialEq, E: PartialEq> PartialEq for OptionalResult<T, E> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Found(s), Found(o)) => *s == *o,
            (Fail(s), Fail(o)) => *s == *o,
            (Empty, Empty) => true,
            (_, _) => false
        }
    }
}

impl<T: PartialEq, E> PartialEq<Option<T>> for OptionalResult<T, E> {
    fn eq(&self, other: &Option<T>) -> bool {
        match (self, other) {
            (Found(s), Some(o)) => *s == *o,
            (Empty, None) => true,
            (_, _) => false
        }
    }
}

impl<T: PartialEq, E: PartialEq> PartialEq<Option<Result<T, E>>> for OptionalResult<T, E> {
    fn eq(&self, other: &Option<Result<T, E>>) -> bool {
        match (self, other) {
            (Found(s), Some(Ok(o))) => *s == *o,
            (Fail(s), Some(Err(o))) => *s == *o,
            (Empty, None) => true,
            (_, _) => false
        }
    }
}

impl<T: PartialEq, E: PartialEq> PartialEq<Result<T, E>> for OptionalResult<T, E> {
    fn eq(&self, other: &Result<T, E>) -> bool {
        match (self, other) {
            (Found(s), Ok(o)) => *s == *o,
            (Fail(s), Err(o)) => *s == *o,
            (_, _) => false
        }
    }
}

impl<T: PartialEq, E: PartialEq> PartialEq<Result<Option<T>, E>> for OptionalResult<T, E> {
    fn eq(&self, other: &Result<Option<T>, E>) -> bool {
        match (self, other) {
            (Found(s), Ok(Some(o))) => *s == *o,
            (Fail(s), Err(o)) => *s == *o,
            (Empty, Ok(None)) => true,
            (_, _) => false
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
        assert_eq!(Result::from(OR::Found("test")), Ok(Some("test")));
        assert_eq!(Result::from(OR::Fail("test")), Err("test"));
        assert_eq!(Result::from(OR::Empty), Ok(None));
    }

    #[test]
    fn it_should_convert_detector_result_into_option() {
        assert_eq!(Option::from(OR::Found("test")), Some(Ok("test")));
        assert_eq!(Option::from(OR::Fail("test")), Some(Err("test")));
        assert_eq!(Option::from(OR::Empty), Option::<Result<_, _>>::None);
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
