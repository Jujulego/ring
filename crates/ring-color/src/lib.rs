#[cfg(test)]
mod mock_supports_color;

use rgb::RGB;

#[cfg(feature = "ansi")]
use ansi_colours::ansi256_from_rgb;

#[cfg(feature = "owo-colors")]
use owo_colors::{DynColors, Style, XtermColors};

#[cfg(not(test))]
#[cfg(feature = "ansi")]
use supports_color::on as supports_color_on;

#[cfg(test)]
use mock_supports_color::on as supports_color_on;

////////////////////////////////////////////////////////////////////////////////
// ColorLabel
////////////////////////////////////////////////////////////////////////////////

/// Text label for the color
/// Mapped to an ANSI basic color
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ColorLabel {
    Black, Grey, White,
    Red,         BrightRed,
    Green,       BrightGreen,
    Yellow,      BrightYellow,
    Blue,        BrightBlue,
    Magenta,     BrightMagenta,
    Cyan,        BrightCyan,
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
            ColorLabel::Grey => owo_colors::AnsiColors::BrightBlack,
            ColorLabel::BrightRed => owo_colors::AnsiColors::BrightRed,
            ColorLabel::BrightGreen => owo_colors::AnsiColors::BrightGreen,
            ColorLabel::BrightYellow => owo_colors::AnsiColors::BrightYellow,
            ColorLabel::BrightBlue => owo_colors::AnsiColors::BrightBlue,
            ColorLabel::BrightMagenta => owo_colors::AnsiColors::BrightMagenta,
            ColorLabel::BrightCyan => owo_colors::AnsiColors::BrightCyan,
        }
    }
}

#[cfg(feature = "owo-colors")]
impl From<ColorLabel> for owo_colors::AnsiColors {
    fn from(value: ColorLabel) -> Self {
        value.to_owo()
    }
}

#[cfg(feature = "crossterm")]
impl From<ColorLabel> for crossterm::style::Color {
    fn from(value: ColorLabel) -> Self {
        match value {
            ColorLabel::Black => crossterm::style::Color::Black,
            ColorLabel::Red => crossterm::style::Color::DarkRed,
            ColorLabel::Green => crossterm::style::Color::DarkGreen,
            ColorLabel::Yellow => crossterm::style::Color::DarkYellow,
            ColorLabel::Blue => crossterm::style::Color::DarkBlue,
            ColorLabel::Magenta => crossterm::style::Color::DarkMagenta,
            ColorLabel::Cyan => crossterm::style::Color::DarkCyan,
            ColorLabel::White => crossterm::style::Color::White,
            ColorLabel::Grey => crossterm::style::Color::Grey,
            ColorLabel::BrightRed => crossterm::style::Color::Red,
            ColorLabel::BrightGreen => crossterm::style::Color::Green,
            ColorLabel::BrightYellow => crossterm::style::Color::Yellow,
            ColorLabel::BrightBlue => crossterm::style::Color::Blue,
            ColorLabel::BrightMagenta => crossterm::style::Color::Magenta,
            ColorLabel::BrightCyan => crossterm::style::Color::Cyan,
        }
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
    pub const fn new(label: ColorLabel, rgb: RGB<u8>) -> Self {
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
        supports_color_on(stream).map(|support| {
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

#[cfg(feature = "crossterm")]
impl StableColor {
    pub fn stylize<T: crossterm::style::Stylize>(&self, val: T) -> T::Styled {
        self.stylize_for(val, supports_color::Stream::Stdout)
    }
    
    pub fn stylize_for<T: crossterm::style::Stylize>(&self, val: T, stream: supports_color::Stream) -> T::Styled {
        if let Some(support) = supports_color_on(stream) {
            if support.has_16m {
                val.with(crossterm::style::Color::Rgb { 
                    r: self.rgb().r,
                    g: self.rgb().g,
                    b: self.rgb().b,
                })
            } else if support.has_256 {
                val.with(crossterm::style::Color::AnsiValue(ansi256_from_rgb(self.rgb)))
            } else {
                val.with(self.label.into())
            }
        } else {
            val.stylize()
        }
    }
}

impl From<&StableColor> for RGB<u8> {
    fn from(stable_color: &StableColor) -> Self {
        stable_color.rgb
    }
}

#[cfg(feature = "owo-colors")]
impl From<&StableColor> for Style {
    fn from(stable_color: &StableColor) -> Self {
        stable_color.to_owo()
            .map(|owo| Style::new().color(owo))
            .unwrap_or_default()
    }
}

////////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use owo_colors::AnsiColors;

    #[test]
    fn it_should_return_a_rgb_color() {
        let color = StableColor::new(ColorLabel::Blue, (0, 0, 255).into());

        mock_supports_color::set_has_16m();
        assert_eq!(color.to_owo(), Some(DynColors::Rgb(0, 0, 255)));
    }

    #[test]
    fn it_should_return_a_xterm_color() {
        let color = StableColor::new(ColorLabel::Blue, (0, 0, 255).into());

        mock_supports_color::set_has_256();
        assert_eq!(color.to_owo(), Some(DynColors::Xterm(XtermColors::Blue)));
    }

    #[test]
    fn it_should_return_an_ansi_color() {
        let color = StableColor::new(ColorLabel::Blue, (0, 0, 255).into());

        mock_supports_color::set_has_basic();
        assert_eq!(color.to_owo(), Some(DynColors::Ansi(AnsiColors::Blue)));
    }
}