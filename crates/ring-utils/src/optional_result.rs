use crate::OptionalResult::{Empty, Fail, Found};

/// Combination of Option and Result
///
/// # Examples
///
/// `OptionalResult<T, E>` is the same as `Option<Result<T, E>>`
/// ```
/// use ring_utils::OptionalResult::{self, *};
///
/// assert_eq!(Found::<&str, ()>("test"), Some(Ok("test")));
/// assert_eq!(Empty::<&str, ()>, None::<Result<_, _>>);
/// assert_eq!(Fail::<&str, ()>(()), Some(Err(())));
/// ```
///
/// `OptionalResult<T, E>` is the same as `Result<Option<T>, E>`
/// ```
/// use ring_utils::OptionalResult::{self, *};
///
/// assert_eq!(Found::<&str, ()>("test"), Ok(Some("test")));
/// assert_eq!(Empty::<&str, ()>, Ok(None));
/// assert_eq!(Fail::<&str, ()>(()), Err::<Option<_>, _>(()));
/// ```
#[derive(Debug, Eq)]
#[must_use = "this `OptionalResult` may be an `Fail` variant, which should be handled"]
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
    #[inline]
    pub fn and_then<U, R: Into<OptionalResult<U, E>>>(self, f: impl FnOnce(T) -> R) -> OptionalResult<U, E> {
        match self {
            Found(val) => f(val).into(),
            Fail(err) => Fail(err),
            Empty => Empty,
        }
    }

    /// Returns [`Empty`] if the optional result is [`Empty`], [`Fail`] if it is [`Fail`] otherwise
    /// calls `predicate` with the wrapped value and returns:
    ///
    /// - [`Found(val)`] if `predicate` returns `true` (where `val` is the wrapped value), and
    /// - [`Empty`] if `predicate` returns `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_utils::OptionalResult::{self, *};
    ///
    /// fn is_even(n: &i32) -> bool {
    ///     n % 2 == 0
    /// }
    ///
    /// assert_eq!(Found::<_, ()>(2).filter(is_even), Found(2));
    /// assert_eq!(Found::<_, ()>(1).filter(is_even), Empty);
    /// assert_eq!(Empty::<_, ()>.filter(is_even), Empty);
    /// assert_eq!(Fail::<_, ()>(()).filter(is_even), Fail(()));
    /// ```
    #[inline]
    pub fn filter<F>(self, predicate: F) -> OptionalResult<T, E>
    where
        F: FnOnce(&T) -> bool,
    {
        match self {
            Found(val) if predicate(&val) => Found(val),
            Found(_) | Empty => Empty,
            Fail(err) => Fail(err),
        }
    }

    /// Calls a function with a reference to the contained value if [`Found`]
    ///
    /// Returns the original optional result
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_utils::OptionalResult::{self, *};
    ///
    /// // prints "hello world"
    /// let result = Found::<&str, ()>("world").inspect(|txt| println!("hello {txt}"));
    ///
    /// // prints nothing
    /// let result = Empty::<&str, ()>.inspect(|txt| println!("hello {txt}"));
    /// let result = Fail::<&str, ()>(()).inspect(|txt| println!("hello {txt}"));
    /// ```
    #[inline]
    pub fn inspect(self, f: impl FnOnce(&T)) -> OptionalResult<T, E> {
        if let Found(ref val) = self {
            f(val);
        }

        self
    }

    /// Returns `true` if self is an [`Empty`] value
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_utils::OptionalResult;
    /// use ring_utils::OptionalResult::{Empty, Fail, Found};
    ///
    /// let optional_result: OptionalResult<&str, ()> = Empty;
    /// assert_eq!(optional_result.is_empty(), true);
    ///
    /// let optional_result: OptionalResult<&str, ()> = Found("test");
    /// assert_eq!(optional_result.is_empty(), false);
    ///
    /// let optional_result: OptionalResult<&str, ()> = Fail(());
    /// assert_eq!(optional_result.is_empty(), false);
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        matches!(self, Empty)
    }

    /// Returns `true` if self is a [`Fail`] value
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_utils::OptionalResult;
    /// use ring_utils::OptionalResult::{Empty, Fail, Found};
    ///
    /// let optional_result: OptionalResult<&str, ()> = Empty;
    /// assert_eq!(optional_result.is_fail(), false);
    ///
    /// let optional_result: OptionalResult<&str, ()> = Fail(());
    /// assert_eq!(optional_result.is_fail(), true);
    ///
    /// let optional_result: OptionalResult<&str, ()> = Found("test");
    /// assert_eq!(optional_result.is_fail(), false);
    /// ```
    #[inline]
    pub fn is_fail(&self) -> bool {
        matches!(self, Fail(_))
    }


    /// Returns `true` if self is a [`Found`] value
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_utils::OptionalResult;
    /// use ring_utils::OptionalResult::{Empty, Fail, Found};
    ///
    /// let optional_result: OptionalResult<&str, ()> = Empty;
    /// assert_eq!(optional_result.is_found(), false);
    ///
    /// let optional_result: OptionalResult<&str, ()> = Fail(());
    /// assert_eq!(optional_result.is_found(), false);
    ///
    /// let optional_result: OptionalResult<&str, ()> = Found("test");
    /// assert_eq!(optional_result.is_found(), true);
    /// ```
    #[inline]
    pub fn is_found(&self) -> bool {
        matches!(self, Found(_))
    }

    /// Apply a function to the contained value (if [`Found`]) mapping `OptionalResult<T, E>` to
    /// `OptionalResult<U, E>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_utils::OptionalResult::{self, *};
    ///
    /// assert_eq!(Found::<&str, ()>("test").map(|s| s.len()), Found(4));
    /// assert_eq!(Empty::<&str, ()>.map(|s| s.len()), Empty);
    /// assert_eq!(Fail::<&str, ()>(()).map(|s| s.len()), Fail(()));
    /// ```
    #[inline]
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> OptionalResult<U, E> {
        match self {
            Found(data) => Found(f(data)),
            Fail(err) => Fail(err),
            Empty => Empty,
        }
    }

    /// Returns [`Ok`] if the optional result is [`Found`], [`Err`] if it is [`Fail`] otherwise
    /// calls `f` and return the result wrapped in [`Ok`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_utils::OptionalResult::{self, *};
    ///
    /// assert_eq!(Found::<i32, ()>(2).result_or_else(|| 42), Ok(2));
    /// assert_eq!(Empty::<i32, ()>.result_or_else(|| 42), Ok(42));
    /// assert_eq!(Fail::<i32, ()>(()).result_or_else(|| 42), Err(()));
    /// ```
    #[inline]
    pub fn result_or_else(self, f: impl FnOnce() -> T) -> Result<T, E> {
        match self {
            Found(val) => Ok(val),
            Fail(err) => Err(err),
            Empty => Ok(f()),
        }
    }

    /// Returns [`Ok`] if the optional result is [`Found`], [`Err`] if it is [`Fail`] otherwise
    /// returns `val` wrapped in [`Ok`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_utils::OptionalResult::{self, *};
    ///
    /// assert_eq!(Found::<i32, ()>(2).result_or(42), Ok(2));
    /// assert_eq!(Empty::<i32, ()>.result_or(42), Ok(42));
    /// assert_eq!(Fail::<i32, ()>(()).result_or(42), Err(()));
    /// ```
    #[inline]
    pub fn result_or(self, val: T) -> Result<T, E> {
        match self {
            Found(val) => Ok(val),
            Fail(err) => Err(err),
            Empty => Ok(val),
        }
    }

    /// Returns [`Ok`] if the optional result is [`Found`], [`Err`] if it is [`Fail`] otherwise
    /// calls `Default::default` and return the result wrapped in [`Ok`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_utils::OptionalResult::{self, *};
    ///
    /// assert_eq!(Found::<i32, ()>(2).result_or_default(), Ok(2));
    /// assert_eq!(Empty::<i32, ()>.result_or_default(), Ok(0));
    /// assert_eq!(Fail::<i32, ()>(()).result_or_default(), Err(()));
    /// ```
    #[inline]
    pub fn result_or_default(self) -> Result<T, E>
    where
        T: Default
    {
        self.result_or_else(T::default)
    }
}

/// Default value for OptionalResult is [`Empty`]
///
/// # Examples
///
/// ```
/// use ring_utils::OptionalResult::{self, *};
///
/// let result: OptionalResult<(), ()> = Default::default();
/// assert_eq!(result, Empty);
/// ```
impl<T, E> Default for OptionalResult<T, E> {
    #[inline]
    fn default() -> Self {
        Empty
    }
}

impl<T, E> From<Result<T, E>> for OptionalResult<T, E> {
    #[inline]
    fn from(res: Result<T, E>) -> Self {
        match res {
            Ok(val) => Found(val),
            Err(err) => Fail(err),
        }
    }
}

impl<T, E> From<OptionalResult<T, E>> for Result<Option<T>, E> {
    #[inline]
    fn from(res: OptionalResult<T, E>) -> Self {
        match res {
            Found(val) => Ok(Some(val)),
            Fail(err) => Err(err),
            Empty => Ok(None),
        }
    }
}

impl<T, E> From<Option<T>> for OptionalResult<T, E> {
    #[inline]
    fn from(opt: Option<T>) -> Self {
        match opt {
            Some(val) => Found(val),
            None => Empty,
        }
    }
}

impl<T, E> From<OptionalResult<T, E>> for Option<Result<T, E>> {
    #[inline]
    fn from(res: OptionalResult<T, E>) -> Self {
        match res {
            Found(val) => Some(Ok(val)),
            Fail(err) => Some(Err(err)),
            Empty => None,
        }
    }
}

impl<T: PartialEq, E: PartialEq> PartialEq for OptionalResult<T, E> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Found(s), Found(o)) => *s == *o,
            (Fail(s), Fail(o)) => *s == *o,
            (Empty, Empty) => true,
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
    fn inspect_should_call_f_only_on_found() {
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
