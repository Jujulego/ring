use rgb::{Rgb, RGB};

#[cfg(feature = "owo-colors")]
use ansi_colours::ansi256_from_rgb;

#[cfg(feature = "owo-colors")]
use owo_colors::{DynColors, XtermColors};

////////////////////////////////////////////////////////////////////////////////
// ColorLabel
////////////////////////////////////////////////////////////////////////////////

/// Text label for the color
/// Mapped to an ANSI basic color
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ColorLabel {
    Black,      BrightBlack,
    Red,        BrightRed,
    Green,      BrightGreen,
    Yellow,     BrightYellow,
    Blue,       BrightBlue,
    Magenta,    BrightMagenta,
    Cyan,       BrightCyan,
    White,      BrightWhite,
}

#[cfg(feature = "owo-colors")]
impl ColorLabel {
    pub fn to_owo(&self) -> owo_colors::AnsiColors {
        match self {
            ColorLabel::Black => owo_colors::AnsiColors::Black,
            ColorLabel::Red => owo_colors::AnsiColors::Red,
            ColorLabel::Green => owo_colors::AnsiColors::Green,
            ColorLabel::Yellow => owo_colors::AnsiColors::Yellow,
            ColorLabel::Blue => owo_colors::AnsiColors::Blue,
            ColorLabel::Magenta => owo_colors::AnsiColors::Magenta,
            ColorLabel::Cyan => owo_colors::AnsiColors::Cyan,
            ColorLabel::White => owo_colors::AnsiColors::White,
            ColorLabel::BrightBlack => owo_colors::AnsiColors::BrightBlack,
            ColorLabel::BrightRed => owo_colors::AnsiColors::BrightRed,
            ColorLabel::BrightGreen => owo_colors::AnsiColors::BrightGreen,
            ColorLabel::BrightYellow => owo_colors::AnsiColors::BrightYellow,
            ColorLabel::BrightBlue => owo_colors::AnsiColors::BrightBlue,
            ColorLabel::BrightMagenta => owo_colors::AnsiColors::BrightMagenta,
            ColorLabel::BrightCyan => owo_colors::AnsiColors::BrightCyan,
            ColorLabel::BrightWhite => owo_colors::AnsiColors::BrightWhite,
        }
    }
}

#[cfg(feature = "owo-colors")]
impl From<ColorLabel> for owo_colors::AnsiColors {
    fn from(value: ColorLabel) -> Self {
        value.to_owo()
    }
}

////////////////////////////////////////////////////////////////////////////////
// StableColor
////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StableColor {
    rgb: RGB<u8>,
    label: ColorLabel
}

impl StableColor {
    #[inline]
    pub fn new<C: Into<Rgb<u8>>>(label: ColorLabel, color: C) -> Self {
        StableColor::_new(label, color.into())
    }
    
    
    fn _new(label: ColorLabel, rgb: RGB<u8>) -> Self {
        StableColor { rgb, label }
    }

    pub fn label(&self) -> &ColorLabel {
        &self.label
    }

    pub fn rgb(&self) -> &RGB<u8> {
        &self.rgb
    }
}

#[cfg(feature = "owo-colors")]
impl StableColor {
    pub fn to_owo(&self) -> Option<DynColors> {
        self.to_owo_for_stream(supports_color::Stream::Stdout)
    }

    pub fn to_owo_for_stream(&self, stream: supports_color::Stream) -> Option<DynColors> {
        supports_color::on(stream).map(|support| {
            if support.has_16m {
                DynColors::Rgb(self.rgb.r, self.rgb.g, self.rgb.b)
            } else if support.has_256 {
                DynColors::Xterm(XtermColors::from(ansi256_from_rgb(self.rgb)))
            } else {
                DynColors::Ansi(self.label.into())
            }
        })
    }
}

impl From<StableColor> for RGB<u8> {
    fn from(stable_color: StableColor) -> Self {
        stable_color.rgb
    }
}