use std::fmt::{Display, Formatter, Error};
use crate::{SGREffect, ANSIForegroundColor, ANSIBackgroundColor};
use crate::traits::Coded;

// Terminal Style -------------------------------------------------------------------------------- /

/// Stores a custom style for terminal text.
/// This creates the escape command needed to change terminal text color.
/// [ANSI Escape Codes](https://en.wikipedia.org/wiki/ANSI_escape_code)
///
/// # Examples
/// ```
/// use terminal_text_styler::{TerminalStyle, SGREffect, ANSIForegroundColor};
///
/// // Manual with codes
/// let yellow_manual = TerminalStyle::from(vec![0, 93]);
///
/// // Using enums
/// let yellow = TerminalStyle::new(vec![SGREffect::Normal], Some(ANSIForegroundColor::BrightYellow), None);
/// let no_color = TerminalStyle::new_empty();
/// assert_eq!(yellow.command(), "\u{001B}[0;93m");
/// assert_eq!(no_color.command(), "\u{001B}[0m");
/// assert_eq!(yellow.wrap("Hello, World!"), "\u{001B}[0;93mHello, World!\u{001B}[0m");
/// ```
#[derive(Debug)]
pub struct TerminalStyle {
    /// List of raw ANSI SGR escape codes used to build the style
    codes: Vec<u8>,

    /// The built command string with escape code
    command: String,
}

impl TerminalStyle {

    /// List of raw ANSI SGR escape codes used to build the style
    pub fn codes(&self) -> &Vec<u8> {
        &self.codes
    }

    /// The built command string with escape code
    pub fn command(&self) -> &str {
        &self.command
    }

    /// Gets a list of all applied SGR effect styles
    pub fn styles(&self) -> Vec<SGREffect> {
        self.codes.iter().map(|code| SGREffect::from(code)).collect()
    }

    /// Looks up the foreground color (Note: This is not supported for ANSI 256-colors and will return `None`)
    pub fn foreground(&self) -> Option<ANSIForegroundColor> {
        let mut possible_foreground: Option<ANSIForegroundColor> = None;
        for code in self.codes().iter() {
            if let Some(foreground) = ANSIForegroundColor::from(*code) {
                possible_foreground = Some(foreground);
            }
        }
        possible_foreground
    }

    /// Looks up the background color (Note: This is not supported for ANSI 256-colors and will return `None`)
    pub fn background(&self) -> Option<ANSIBackgroundColor> {
        let mut possible_background: Option<ANSIBackgroundColor> = None;
        for code in self.codes().iter() {
            if let Some(background) = ANSIBackgroundColor::from(*code) {
                possible_background = Some(background);
            }
        }
        possible_background
    }

    // Init ------------------------------------------------------------------------------ /

    /// Creates a new terminal color with given escape codes.
    pub fn from(codes: Vec<u8>) -> Self {
        let command = Self::make_command(&codes);
        TerminalStyle {
            codes,
            command,
        }
    }

    /// Creates a new empty terminal color (aka no color).
    pub fn new_empty() -> Self {
        Self::new(vec![SGREffect::Normal], None, None)
    }

    /// Creates a new terminal color with given options.
    pub fn new(
        effects: Vec<SGREffect>,
        foreground: Option<ANSIForegroundColor>,
        background: Option<ANSIBackgroundColor>
    ) -> Self {
        let mut codes: Vec<u8> = effects.iter().map(|effect| effect.code()).collect();
        if let Some(foreground) = foreground {
            codes.push(foreground.code());
            match foreground {
                ANSIForegroundColor::ANSI256(ansi256) => {
                    codes.push(5);
                    codes.push(ansi256);
                }
                _ => ()
            }
        }
        if let Some(background) = background {
            codes.push(background.code());
            match background {
                ANSIBackgroundColor::ANSI256(ansi256) => {
                    codes.push(5);
                    codes.push(ansi256);
                }
                _ => ()
            }
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
    /// let style = TerminalStyle::new(vec![SGREffect::Bold], Some(ANSIForegroundColor::BrightYellow), None);
    /// let output = style.wrap("This will be bold and highlighted in bright yellow.");
    /// println!("{}", output);
    /// ```
    pub fn wrap(&self, text: &str) -> String {
        let start = self.command();
        let empty = TerminalStyle::new_empty();
        let end = empty.command();
        format!("{}{}{}", start, text, end)
    }

    // Private instance methods ------------------------------------------------------------------ /

    /// This formats the ANSI escape code string that switches the terminal color.
    fn make_command(codes: &Vec<u8>) -> String {
        let code_strings: Vec<String> = codes.iter().map(|code| code.to_string()).collect();
        let code = code_strings.join(";");
        let beginning = "\u{001B}[";
        format!("{}{}m", beginning, code)
    }
}

impl Display for TerminalStyle {

    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error> {
        let command = self.command();
        write!(formatter, "{}", command)
    }
}

impl Clone for TerminalStyle {

    fn clone(&self) -> Self {
        Self::from(self.codes.clone())
    }
}

impl PartialEq for TerminalStyle {

    fn eq(&self, other: &Self) -> bool {
        self.command == other.command
    }
}

impl Eq for TerminalStyle {}

impl TerminalStyle {

    // Presets ----------------------------------------------------------------------------------- /

    // No color / Reset --- /
    pub fn no_color() -> Self { Self::new_empty() }
    pub fn reset() -> Self { Self::new_empty() }

    // Normal --- /
    pub fn black() -> Self { Self::new(vec![SGREffect::Normal], Some(ANSIForegroundColor::Black), None) }
    pub fn red() -> Self { Self::new(vec![SGREffect::Normal], Some(ANSIForegroundColor::Red), None) }
    pub fn green() -> Self { Self::new(vec![SGREffect::Normal], Some(ANSIForegroundColor::Green), None) }
    pub fn yellow() -> Self { Self::new(vec![SGREffect::Normal], Some(ANSIForegroundColor::Yellow), None) }
    pub fn blue() -> Self { Self::new(vec![SGREffect::Normal], Some(ANSIForegroundColor::Blue), None) }
    pub fn magenta() -> Self { Self::new(vec![SGREffect::Normal], Some(ANSIForegroundColor::Magenta), None) }
    pub fn cyan() -> Self { Self::new(vec![SGREffect::Normal], Some(ANSIForegroundColor::Cyan), None) }
    pub fn white() -> Self { Self::new(vec![SGREffect::Normal], Some(ANSIForegroundColor::White), None) }

    // Bold and Bright --- /
    pub fn bright_black() -> Self { Self::new(vec![SGREffect::Bold], Some(ANSIForegroundColor::BrightBlack), None) }
    pub fn bright_red() -> Self { Self::new(vec![SGREffect::Bold], Some(ANSIForegroundColor::BrightRed), None) }
    pub fn bright_green() -> Self { Self::new(vec![SGREffect::Bold], Some(ANSIForegroundColor::BrightGreen), None) }
    pub fn bright_yellow() -> Self { Self::new(vec![SGREffect::Bold], Some(ANSIForegroundColor::BrightYellow), None) }
    pub fn bright_blue() -> Self { Self::new(vec![SGREffect::Bold], Some(ANSIForegroundColor::BrightBlue), None) }
    pub fn bright_magenta() -> Self { Self::new(vec![SGREffect::Bold], Some(ANSIForegroundColor::BrightMagenta), None) }
    pub fn bright_cyan() -> Self { Self::new(vec![SGREffect::Bold], Some(ANSIForegroundColor::BrightCyan), None) }
    pub fn bright_white() -> Self { Self::new(vec![SGREffect::Bold], Some(ANSIForegroundColor::BrightWhite), None) }

    // Italic --- /
    pub fn italic_black() -> Self { Self::new(vec![SGREffect::Italic], Some(ANSIForegroundColor::Black), None) }
    pub fn italic_red() -> Self { Self::new(vec![SGREffect::Italic], Some(ANSIForegroundColor::Red), None) }
    pub fn italic_green() -> Self { Self::new(vec![SGREffect::Italic], Some(ANSIForegroundColor::Green), None) }
    pub fn italic_yellow() -> Self { Self::new(vec![SGREffect::Italic], Some(ANSIForegroundColor::Yellow), None) }
    pub fn italic_blue() -> Self { Self::new(vec![SGREffect::Italic], Some(ANSIForegroundColor::Blue), None) }
    pub fn italic_magenta() -> Self { Self::new(vec![SGREffect::Italic], Some(ANSIForegroundColor::Magenta), None) }
    pub fn italic_cyan() -> Self { Self::new(vec![SGREffect::Italic], Some(ANSIForegroundColor::Cyan), None) }
    pub fn italic_white() -> Self { Self::new(vec![SGREffect::Italic], Some(ANSIForegroundColor::White), None) }

    // Backgrounds --- /
    pub fn red_background() -> Self { Self::new(vec![SGREffect::Bold], Some(ANSIForegroundColor::BrightWhite), Some(ANSIBackgroundColor::Red)) }
    pub fn blue_background() -> Self { Self::new(vec![SGREffect::Bold], Some(ANSIForegroundColor::BrightWhite), Some(ANSIBackgroundColor::Blue)) }
    pub fn green_background() -> Self { Self::new(vec![SGREffect::Normal], Some(ANSIForegroundColor::Black), Some(ANSIBackgroundColor::Green)) }
    pub fn yellow_background() -> Self { Self::new(vec![SGREffect::Normal], Some(ANSIForegroundColor::Black), Some(ANSIBackgroundColor::BrightYellow)) }

    // Misc. --- /
    pub fn normal_bright_yellow() -> Self { Self::new(vec![SGREffect::Normal], Some(ANSIForegroundColor::BrightYellow), None) }
    pub fn blink() -> Self { Self::new(vec![SGREffect::SlowBlink], None, None) }
}

// Reference ------------------------------------------------------------------------------------- /

// Source: https://stackoverflow.com/a/5947802/3055803

// ANSI Escape Codes:
// ---------------------------------------- /
// Black        0;30     Dark Gray     1;30
// Red          0;31     Light Red     1;31
// Green        0;32     Light Green   1;32
// Brown/Orange 0;33     Yellow        1;33
// Blue         0;34     Light Blue    1;34
// Purple       0;35     Light Purple  1;35
// Cyan         0;36     Light Cyan    1;36
// Light Gray   0;37     White         1;37

// ---------------------------------------- /
// RED='\033[0;31m'
// NC='\033[0m' # No Color
// printf "I ${RED}love${NC} Stack Overflow\n"

// Source: https://stackoverflow.com/q/40583721/3055803

// let redColor = "\u{001B}[0;31m"
// let message = "Some Message"
// print(redColor + message)
// print("\(redColor)\(message)")

// Tests ----------------------------------------------------------------------------------------- /

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_that_code_generates_correctly() {
        let manual = TerminalStyle::from(vec![0,33,41]);
        let yellow = TerminalStyle::new(
            vec![SGREffect::Bold],
            Some(ANSIForegroundColor::BrightYellow),
            None,
        );
        let yellow_with_background = TerminalStyle::new(
            vec![SGREffect::Normal],
            Some(ANSIForegroundColor::Yellow),
            Some(ANSIBackgroundColor::Red),
        );
        let multi_style = TerminalStyle::new(
            vec![SGREffect::Bold, SGREffect::Italic, SGREffect::SlowBlink],
            Some(ANSIForegroundColor::Blue),
            None,
        );
        let multi_style_02 = TerminalStyle::new(
            vec![SGREffect::Bold, SGREffect::Italic, SGREffect::SlowBlink],
            None,
            Some(ANSIBackgroundColor::White),
        );
        let multi_style_03 = TerminalStyle::new(
            vec![SGREffect::Bold, SGREffect::Italic, SGREffect::SlowBlink],
            None,
            None,
        );
        let color_256 = TerminalStyle::new(
            vec![SGREffect::Bold],
            Some(ANSIForegroundColor::ANSI256(183)),
            Some(ANSIBackgroundColor::ANSI256(190)),
        );
        let no_color = TerminalStyle::new_empty();
        assert_eq!(manual.command(), "\u{001B}[0;33;41m");
        assert_eq!(yellow.command(), "\u{001B}[1;93m");
        assert_eq!(yellow_with_background.command(), "\u{001B}[0;33;41m");
        assert_eq!(multi_style.command(), "\u{001B}[1;3;5;34m");
        assert_eq!(multi_style_02.command(), "\u{001B}[1;3;5;47m");
        assert_eq!(multi_style_03.command(), "\u{001B}[1;3;5m");
        assert_eq!(color_256.command(), "\u{001B}[1;38;5;183;48;5;190m");
        assert_eq!(no_color.command(), "\u{001B}[0m");
        assert_eq!(manual, yellow_with_background);
        assert_eq!(yellow, TerminalStyle::bright_yellow());
        assert_eq!(no_color, TerminalStyle::no_color());
        assert_eq!(no_color, TerminalStyle::reset());
        assert_eq!(TerminalStyle::no_color(), TerminalStyle::reset());
    }

    #[test]
    fn test_color_lookup() {
        let yellow = TerminalStyle::new(
            vec![SGREffect::Bold],
            Some(ANSIForegroundColor::BrightYellow),
            None,
        );
        let yellow_with_background = TerminalStyle::new(
            vec![SGREffect::Normal],
            Some(ANSIForegroundColor::Yellow),
            Some(ANSIBackgroundColor::Red),
        );
        assert_eq!(yellow.foreground(), Some(ANSIForegroundColor::BrightYellow));
        assert_eq!(yellow.background(), None);
        assert_eq!(yellow_with_background.background(), Some(ANSIBackgroundColor::Red));
    }

    #[test]
    fn test_style_lookup() {
        let yellow = TerminalStyle::new(
            vec![SGREffect::Bold],
            Some(ANSIForegroundColor::BrightYellow),
            None,
        );
        let multi_style = TerminalStyle::new(
            vec![SGREffect::Bold, SGREffect::Italic, SGREffect::SlowBlink],
            Some(ANSIForegroundColor::Blue),
            None,
        );
        let multi_style_03 = TerminalStyle::new(
            vec![SGREffect::Bold, SGREffect::Italic, SGREffect::SlowBlink],
            None,
            None,
        );
        assert_eq!(yellow.styles().len(), 2);
        assert_eq!(yellow.styles()[0], SGREffect::Bold);
        assert_eq!(yellow.styles()[1], SGREffect::ByCode(93));
        assert_eq!(multi_style.styles().len(), 4);
        assert_eq!(multi_style_03.styles().len(), 3);
    }

    #[test]
    fn test_that_content_can_be_wrapped() {
        let yellow = TerminalStyle::new(
            vec![SGREffect::Bold],
            Some(ANSIForegroundColor::BrightYellow),
            None,
        );
        let no_color = TerminalStyle::new_empty();
        let wrapped = yellow.wrap("Hello, World!");
        assert_eq!(&wrapped, "\u{001B}[1;93mHello, World!\u{001B}[0m");
        assert_eq!(wrapped, format!("{}Hello, World!{}", yellow, no_color));
    }
}
