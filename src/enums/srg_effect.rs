use crate::traits::{Coded};
use std::fmt::{Display, Formatter, Error};

/// SGR (Select Graphic Rendition) sets display attributes.
/// [ANSI Escape Codes](https://en.wikipedia.org/wiki/ANSI_escape_code)
#[derive(Debug)]
pub enum SGREffect {
    Normal,
    Bold,
    Faint,
    Italic,
    Underline,
    SlowBlink,
    RapidBlink,
    CrossedOut,
    ByCode(u8),
}

impl Coded for SGREffect {

    /// ANSI escape code
    fn code(&self) -> u8 {
        match self {
            SGREffect::Normal => 0,
            SGREffect::Bold => 1,
            SGREffect::Faint => 2,
            SGREffect::Italic => 3,
            SGREffect::Underline => 4,
            SGREffect::SlowBlink => 5,
            SGREffect::RapidBlink => 6,
            SGREffect::CrossedOut => 9,
            SGREffect::ByCode(code) => *code,
        }
    }
}

impl SGREffect {

    /// Makes a new instance from ANSI escape code
    /// [ANSI Escape Codes](https://en.wikipedia.org/wiki/ANSI_escape_code)
    pub fn from(code: &u8) -> Self {
        match code {
            0 => Self::Normal,
            1 => Self::Bold,
            2 => Self::Faint,
            3 => Self::Italic,
            4 => Self::Underline,
            5 => Self::SlowBlink,
            6 => Self::RapidBlink,
            9 => Self::CrossedOut,
            _ => Self::ByCode(*code),
        }
    }

    /// String representation
    fn description(&self) -> String {
        match self {
            SGREffect::Normal => String::from("normal/reset"),
            SGREffect::Bold => String::from("bold"),
            SGREffect::Faint => String::from("faint"),
            SGREffect::Italic => String::from("italic"),
            SGREffect::Underline => String::from("underline"),
            SGREffect::SlowBlink => String::from("slow blink"),
            SGREffect::RapidBlink => String::from("rapid blink"),
            SGREffect::CrossedOut => String::from("crossed-out"),
            SGREffect::ByCode(code) => format!("SGR Code {}", code),
        }
    }
}

impl Display for SGREffect {

    /// String formatter
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let description = self.description();
        write!(f, "{}", description)
    }
}

impl PartialEq for SGREffect {

    fn eq(&self, other: &Self) -> bool {
        self.code() == other.code()
    }
}

impl Eq for SGREffect {}
