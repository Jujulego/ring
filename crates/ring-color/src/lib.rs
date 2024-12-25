#[cfg(test)]
mod mock_supports_color;

use rgb::RGB;

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
                val.with(crossterm::style::Color::AnsiValue(ansi_colours::ansi256_from_rgb(self.rgb)))
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
