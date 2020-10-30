use crate::traits::{Coded};
use std::fmt::{Display, Formatter, Error};

/// ANSI Escape codes for text foreground color.
/// [ANSI Escape Codes](https://en.wikipedia.org/wiki/ANSI_escape_code)
pub enum ANSIForegroundColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    ANSI256(u8),
}

impl Coded for ANSIForegroundColor {

    /// ANSI escape code
    fn get_code(&self) -> u8 {
        match self {
            ANSIForegroundColor::Black => 30,
            ANSIForegroundColor::Red => 31,
            ANSIForegroundColor::Green => 32,
            ANSIForegroundColor::Yellow => 33,
            ANSIForegroundColor::Blue => 34,
            ANSIForegroundColor::Magenta => 35,
            ANSIForegroundColor::Cyan => 36,
            ANSIForegroundColor::White => 37,
            ANSIForegroundColor::BrightBlack => 90,
            ANSIForegroundColor::BrightRed => 91,
            ANSIForegroundColor::BrightGreen => 92,
            ANSIForegroundColor::BrightYellow => 93,
            ANSIForegroundColor::BrightBlue => 94,
            ANSIForegroundColor::BrightMagenta => 95,
            ANSIForegroundColor::BrightCyan => 96,
            ANSIForegroundColor::BrightWhite => 97,
            ANSIForegroundColor::ANSI256(custom) => custom,
        }
    }
}

impl ANSIForegroundColor {

    /// Makes a new instance from ANSI escape code
    /// [ANSI Escape Codes](https://en.wikipedia.org/wiki/ANSI_escape_code)
    ///  **Parameters:**
    /// - `code`: Primary ANSI code
    pub fn from(code: u8) -> Option<Self> {
        match code {
            30 => Some(ANSIForegroundColor::Black),
            31 => Some(ANSIForegroundColor::Red),
            32 => Some(ANSIForegroundColor::Green),
            33 => Some(ANSIForegroundColor::Yellow),
            34 => Some(ANSIForegroundColor::Blue),
            35 => Some(ANSIForegroundColor::Magenta),
            36 => Some(ANSIForegroundColor::Cyan),
            37 => Some(ANSIForegroundColor::White),
            90 => Some(ANSIForegroundColor::BrightBlack),
            91 => Some(ANSIForegroundColor::BrightRed),
            92 => Some(ANSIForegroundColor::BrightGreen),
            93 => Some(ANSIForegroundColor::BrightYellow),
            94 => Some(ANSIForegroundColor::BrightBlue),
            95 => Some(ANSIForegroundColor::BrightMagenta),
            96 => Some(ANSIForegroundColor::BrightCyan),
            97 => Some(ANSIForegroundColor::BrightWhite),
            _ => None,
        }
    }

    /// Makes a new instance from a 256-color ANSI escape code
    /// [ANSI Escape Codes](https://en.wikipedia.org/wiki/ANSI_escape_code)
    ///  **Parameters:**
    /// - `code`: Primary ANSI code
    /// - `ansi256`: Specify an ANSI 256-color code (hint: only relevant when primary code is 48)
    pub fn from_256(code: u8, ansi_256: u8) -> Option<Self> {
        match code {
            38 => Some(ANSIForegroundColor::ANSI256(ansi_256)),
            _ => Self::from(code),
        }
    }

    /// String representation
    fn get_description(&self) -> &str {
        match self {
            ANSIForegroundColor::Black => "Black",
            ANSIForegroundColor::Red => "Red",
            ANSIForegroundColor::Green => "Green",
            ANSIForegroundColor::Yellow => "Yellow",
            ANSIForegroundColor::Blue => "Blue",
            ANSIForegroundColor::Magenta => "Magenta",
            ANSIForegroundColor::Cyan => "Cyan",
            ANSIForegroundColor::White => "White",
            ANSIForegroundColor::BrightBlack => "Bright Black",
            ANSIForegroundColor::BrightRed => "Bright Red",
            ANSIForegroundColor::BrightGreen => "Bright Green",
            ANSIForegroundColor::BrightYellow => "Bright Yellow",
            ANSIForegroundColor::BrightBlue => "Bright Blue",
            ANSIForegroundColor::BrightMagenta => "Bright Magenta",
            ANSIForegroundColor::BrightCyan => "Bright Cyan",
            ANSIForegroundColor::BrightWhite => "Bright White",
            ANSIForegroundColor::ANSI256(custom) => format!("ANSI 256-color ({})", custom),
        }
    }

    /// ANSI escape codes (only for use with 256-color codes)
    pub fn additional_codes(&self) -> Option<(u8, u8)> {
        match self {
            ANSIForegroundColor::ANSI256(ansi_code) => Some((5, *ansi_code)),
            _ => None,
        }
    }
}

impl Display for ANSIForegroundColor {

    /// String formatter
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let description = self.get_description();
        write!(f, "{}", description)
    }
}
