use crate::traits::{Coded};
use std::fmt::{Display, Formatter, Error};

/// ANSI Escape codes for text background color.
/// [ANSI Escape Codes](https://en.wikipedia.org/wiki/ANSI_escape_code)
#[derive(Debug)]
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
    fn code(&self) -> u8 {
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
            ANSIBackgroundColor::ANSI256(_) => 48,
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
    fn description(&self) -> String {
        match self {
            ANSIBackgroundColor::Black => String::from("Black"),
            ANSIBackgroundColor::Red => String::from("Red"),
            ANSIBackgroundColor::Green => String::from("Green"),
            ANSIBackgroundColor::Yellow => String::from("Yellow"),
            ANSIBackgroundColor::Blue => String::from("Blue"),
            ANSIBackgroundColor::Magenta => String::from("Magenta"),
            ANSIBackgroundColor::Cyan => String::from("Cyan"),
            ANSIBackgroundColor::White => String::from("White"),
            ANSIBackgroundColor::BrightBlack => String::from("Bright Black"),
            ANSIBackgroundColor::BrightRed => String::from("Bright Red"),
            ANSIBackgroundColor::BrightGreen => String::from("Bright Green"),
            ANSIBackgroundColor::BrightYellow => String::from("Bright Yellow"),
            ANSIBackgroundColor::BrightBlue => String::from("Bright Blue"),
            ANSIBackgroundColor::BrightMagenta => String::from("Bright Magenta"),
            ANSIBackgroundColor::BrightCyan => String::from("Bright Cyan"),
            ANSIBackgroundColor::BrightWhite => String::from("Bright White"),
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
        let description = self.description();
        write!(f, "{}", description)
    }
}

impl PartialEq for ANSIBackgroundColor {

    fn eq(&self, other: &Self) -> bool {
        self.code() == other.code()
    }
}

impl Eq for ANSIBackgroundColor {}
