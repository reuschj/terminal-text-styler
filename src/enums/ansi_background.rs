use crate::traits::{Coded};
use std::fmt::{Display, Formatter, Error};

/// ANSI Escape codes for text background color.
/// [ANSI Escape Codes](https://en.wikipedia.org/wiki/ANSI_escape_code)
pub enum ANSIBackgroundColor {
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

impl Coded for ANSIBackgroundColor {

    /// ANSI escape code
    fn get_code(&self) -> u8 {
        match self {
            ANSIBackgroundColor::Black => 40,
            ANSIBackgroundColor::Red => 41,
            ANSIBackgroundColor::Green => 42,
            ANSIBackgroundColor::Yellow => 43,
            ANSIBackgroundColor::Blue => 44,
            ANSIBackgroundColor::Magenta => 45,
            ANSIBackgroundColor::Cyan => 46,
            ANSIBackgroundColor::White => 47,
            ANSIBackgroundColor::BrightBlack => 100,
            ANSIBackgroundColor::BrightRed => 101,
            ANSIBackgroundColor::BrightGreen => 102,
            ANSIBackgroundColor::BrightYellow => 103,
            ANSIBackgroundColor::BrightBlue => 104,
            ANSIBackgroundColor::BrightMagenta => 105,
            ANSIBackgroundColor::BrightCyan => 106,
            ANSIBackgroundColor::BrightWhite => 107,
            ANSIBackgroundColor::ANSI256(custom) => custom,
        }
    }
}

impl ANSIBackgroundColor {

    /// Makes a new instance from ANSI escape code
    /// [ANSI Escape Codes](https://en.wikipedia.org/wiki/ANSI_escape_code)
    ///  **Parameters:**
    /// - `code`: Primary ANSI code
    pub fn from(code: u8) -> Option<Self> {
        match code {
            40 => Some(ANSIBackgroundColor::Black),
            41 => Some(ANSIBackgroundColor::Red),
            42 => Some(ANSIBackgroundColor::Green),
            43 => Some(ANSIBackgroundColor::Yellow),
            44 => Some(ANSIBackgroundColor::Blue),
            45 => Some(ANSIBackgroundColor::Magenta),
            46 => Some(ANSIBackgroundColor::Cyan),
            47 => Some(ANSIBackgroundColor::White),
            100 => Some(ANSIBackgroundColor::BrightBlack),
            101 => Some(ANSIBackgroundColor::BrightRed),
            102 => Some(ANSIBackgroundColor::BrightGreen),
            103 => Some(ANSIBackgroundColor::BrightYellow),
            104 => Some(ANSIBackgroundColor::BrightBlue),
            105 => Some(ANSIBackgroundColor::BrightMagenta),
            106 => Some(ANSIBackgroundColor::BrightCyan),
            107 => Some(ANSIBackgroundColor::BrightWhite),
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
            48 => Some(ANSIBackgroundColor::ANSI256(ansi_256)),
            _ => Self::from(code),
        }
    }

    /// String representation
    fn get_description(&self) -> &str {
        match self {
            ANSIBackgroundColor::Black => "Black",
            ANSIBackgroundColor::Red => "Red",
            ANSIBackgroundColor::Green => "Green",
            ANSIBackgroundColor::Yellow => "Yellow",
            ANSIBackgroundColor::Blue => "Blue",
            ANSIBackgroundColor::Magenta => "Magenta",
            ANSIBackgroundColor::Cyan => "Cyan",
            ANSIBackgroundColor::White => "White",
            ANSIBackgroundColor::BrightBlack => "Bright Black",
            ANSIBackgroundColor::BrightRed => "Bright Red",
            ANSIBackgroundColor::BrightGreen => "Bright Green",
            ANSIBackgroundColor::BrightYellow => "Bright Yellow",
            ANSIBackgroundColor::BrightBlue => "Bright Blue",
            ANSIBackgroundColor::BrightMagenta => "Bright Magenta",
            ANSIBackgroundColor::BrightCyan => "Bright Cyan",
            ANSIBackgroundColor::BrightWhite => "Bright White",
            ANSIBackgroundColor::ANSI256(custom) => format!("ANSI 256-color ({})", custom),
        }
    }

    /// ANSI escape codes (only for use with 256-color codes)
    pub fn additional_codes(&self) -> Option<(u8, u8)> {
        match self {
            ANSIBackgroundColor::ANSI256(ansi_code) => Some((5, *ansi_code)),
            _ => None,
        }
    }
}

impl Display for ANSIBackgroundColor {

    /// String formatter
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let description = self.get_description();
        write!(f, "{}", description)
    }
}
