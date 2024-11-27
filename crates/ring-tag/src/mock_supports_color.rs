use std::cell::RefCell;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct ColorLevel {
    pub has_basic: bool,
    pub has_256: bool,
    pub has_16m: bool,
}

thread_local! {
    static RESULT: RefCell<Option<ColorLevel>> = const { RefCell::new(None) };
}

pub fn on(_stream: supports_color::Stream) -> Option<ColorLevel> {
    RESULT.with(|result| {
        *result.borrow()
    })
}

pub fn set_has_256() {
    RESULT.with(|result| {
        *result.borrow_mut() = Some(ColorLevel {
            has_basic: true,
            has_256: true,
            has_16m: false,
        });
    })
}
pub fn set_has_16m() {
    RESULT.with(|result| {
        *result.borrow_mut() = Some(ColorLevel {
            has_basic: true,
            has_256: true,
            has_16m: true,
        });
    })
}