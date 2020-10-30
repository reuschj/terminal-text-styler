use std::fmt::{Display, Formatter, Error};
use crate::styled_terminal_text::StyledTerminalText;
use crate::{SGREffect, ANSIForegroundColor, ANSIBackgroundColor};
use crate::traits::Coded;

// Terminal Color -------------------------------------------------------------------------------- /

/// Stores a custom style for terminal text.
/// This creates the escape command needed to change terminal text color.
/// [ANSI Escape Codes](https://en.wikipedia.org/wiki/ANSI_escape_code)
pub struct TerminalStyle {
    /// List of raw ANSI SGR escape codes used to build the style
    codes: Option<Vec<u8>>,

    /// The built command string with escape code
    command: String,
}

impl TerminalStyle {

    /// List of raw ANSI SGR escape codes used to build the style
    pub fn codes(&self) -> Option<Vec<u8>> {
        codes
    }

    /// The built command string with escape code
    pub fn command(&self) -> String {
        command
    }

    // Init ------------------------------------------------------------------------------ /

    /// Creates a new terminal color with given escape codes.
    pub fn from(codes: Vec<u8>) -> Self {
        let ansi_codes = if codes.is_empty() { None } else { Some(codes) };
        let command = Self::make_command(&ansi_codes);
        TerminalStyle {
            codes: ansi_codes,
            command,
        }
    }

    /// Creates a new empty terminal color (aka no color).
    pub fn new_empty() -> Self {
        Self::new(None, None, None)
    }

    /// Creates a new terminal color with given options.
    pub fn new(
        effects: Option<Vec<SGREffect>>,
        foreground: Option<ANSIForegroundColor>,
        background: Option<ANSIBackgroundColor>
    ) -> Self {
        let mut codes: Vec<u8> = Vec::new();
        if let Some(effects) = effects {
            effects.iter().for_each(|effect| codes.push(effect.get_code()));
        }
        if let Some(foreground) = foreground {
            codes.push(foreground.get_code());
            match foreground {
                ANSIForegroundColor::ANSI256(ansi256) => {
                    codes.push(5);
                    codes.push(ansi256);
                }
                _ => ()
            }
        }
        if let Some(background) = background {
            codes.push(background.get_code());
            ANSIForegroundColor::ANSI256(ansi256) => {
                codes.push(5);
                codes.push(ansi256);
            }
            _ => ()
        }
        Self::from(codes)
    }

    /// Wraps given text with a command to start the custom color at the beginning and ends
    /// the string with a no-color command.
    ///
    /// # Examples
    /// ```
    /// use terminal_text_styler::{TerminalStyle, SGREffect, ANSIForegroundColor};
    ///
    /// let style = TerminalStyle::new(Some(vec![SGREffect::Bold]), Some(ANSIForegroundColor::BrightYellow), None);
    /// let output = style.wrap("This will be bold and highlighted in bright yellow.");
    /// println!("{}", output);
    /// ```
    pub fn wrap(&self, text: &str) -> String {
        let start = self.get_command();
        let end = TerminalStyle::new_empty().get_command();
        format!("{}{}{}", start, text, end)
    }

    // Private instance methods ------------------------------------------------------------------ /

    /// This formats the ANSI escape code string that switches the terminal color.
    fn make_command(codes: &Option<Vec<u8>>) -> String {
        let code = if let Some(codes) = codes {
            let code_strings: Vec<String> = codes.iter().map(|code| code.to_string()).collect();
            code_strings.join(";")
        } else {
            "0"
        };
        let beginning = "\u{001B}[";
        format!("{}{}m", beginning, code)
    }
}

impl Display for TerminalStyle {

    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error> {
        let command = self.get_command();
        write!(formatter, "{}", command)
    }
}

impl Clone for TerminalStyle {

    fn clone(&self) -> Self {
        let Self { codes, .. } = self;
        if let Some(codes) = codes {
            Self::from(codes.clone())
        } else {
            Self::new_empty()
        }
    }
}

impl TerminalStyle {

    // Presets ----------------------------------------------------------------------------------- /

    pub fn no_color() -> Self { Self::new_empty() }
    pub fn black() -> Self { Self::new(0, 30) }
    pub fn red() -> Self { Self::new(0, 31) }
    pub fn green() -> Self { Self::new(0, 32) }
    pub fn brown_orange() -> Self { Self::new(0, 33) }
    pub fn blue() -> Self { Self::new(0, 34) }
    pub fn purple() -> Self { Self::new(0, 35) }
    pub fn cyan() -> Self { Self::new(0, 36) }
    pub fn light_gray() -> Self { Self::new(0, 37) }
    pub fn dark_gray() -> Self { Self::new(0, 30) }
    pub fn light_red() -> Self { Self::new(1, 31) }
    pub fn light_green() -> Self { Self::new(1, 32) }
    pub fn yellow() -> Self { Self::new(1, 33) }
    pub fn light_blue() -> Self { Self::new(1, 34) }
    pub fn light_purple() -> Self { Self::new(1, 35) }
    pub fn light_cyan() -> Self { Self::new(1, 36) }
    pub fn white() -> Self { Self::new(1, 37) }
}

// Tests ----------------------------------------------------------------------------------------- /

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
