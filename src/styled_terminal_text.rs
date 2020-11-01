use crate::terminal_style::TerminalStyle;
use std::fmt::{Display, Error, Formatter};

// Color Terminal Text --------------------------------------------------------------------------- /

/// Holds and displays text that will print with the specified color in a terminal.
///
/// **Example:**
/// ```
/// use terminal_text_styler::{StyledTerminalText, TerminalStyle};
///
/// let greeting = StyledTerminalText::new("Hello, World!", TerminalStyle::bright_yellow());
/// assert_eq!(greeting.output(), "\u{001B}[1;93mHello, World!\u{001B}[0m");
/// ```
#[derive(Debug)]
pub struct StyledTerminalText {
    text: String,
    style: TerminalStyle,
    output: Option<String>,
}

impl StyledTerminalText {

    /// Gets the un-styled original text
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Gets the `TerminalStyle` instance
    pub fn style(&self) -> &TerminalStyle {
        &self.style
    }

    /// Gets output for terminal
    pub fn output(&self) -> &str {
        if let Some(output) = &self.output {
            output
        } else {
            &self.text
        }
    }

    /// Changes text and returns the existing text.
    pub fn change_text_to(&mut self, new_text: &str) -> String {
        let current_text = self.text.clone();
        self.text = String::from(new_text);
        self.update_output();
        current_text
    }

    /// Changes style and returns the existing style.
    pub fn change_style_to(&mut self, new_style: TerminalStyle) -> TerminalStyle {
        let current_color = self.style.clone();
        self.style = new_style;
        self.update_output();
        current_color
    }

    // Init -------------------------------------------------------------------------------------- /

    /// Creates from a string and terminal color
    pub fn new(text: &str, style: TerminalStyle) -> Self {
        let mut new_instance = Self {
            text: String::from(text),
            style,
            output: None, // Just temporary
        };
        new_instance.update_output();
        new_instance
    }

    // Private instance methods ------------------------------------------------------------------ /

    /// Private method that updates the stored output string
    fn update_output(&mut self) {
        self.output = Some(self.style.wrap(&self.text));
    }
}

impl Display for StyledTerminalText {

    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error> {
        write!(formatter, "{}", self.output())
    }
}

impl PartialEq for StyledTerminalText {

    fn eq(&self, other: &Self) -> bool {
        self.output == other.output
    }
}

impl Eq for StyledTerminalText {}

// Tests ----------------------------------------------------------------------------------------- /

#[cfg(test)]
mod tests {
    use super::*;
    use crate::terminal_style::TerminalStyle;

    #[test]
    fn test_styled_terminal_text() {
        let highlighted = StyledTerminalText::new("Hello, World!", TerminalStyle::bright_yellow());
        assert_eq!(highlighted.output(), "\u{001B}[1;93mHello, World!\u{001B}[0m");
        assert_eq!(format!("{}", highlighted), "\u{001B}[1;93mHello, World!\u{001B}[0m");
    }
}

