use crate::terminal_style::TerminalStyle;
use std::fmt::{Display, Error};

// Color Terminal Text --------------------------------------------------------------------------- /

/// Holds and displays text that will print with the specified color in a terminal.
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
        write!(formatter, "{}", self.get_output())
    }
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

