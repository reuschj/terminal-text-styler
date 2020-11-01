use crate::{StyledTerminalText, TerminalStyle};

// Highlight utility functions ------------------------------------------------------------------- /

/// Highlights the given text in the requested terminal color, (defaulting to bright yellow).
///
/// **Parameters:**
/// - `text`: The content to highlight
/// - `style`: The terminal style to highlight with.
///
/// Returns an instance of `StyledTerminalText` to add to string output
pub fn highlight(text: &str, style: TerminalStyle) -> StyledTerminalText {
    StyledTerminalText::new(text, style)
}

// Convenience functions for each preset color
pub fn highlight_black(text: &str) -> StyledTerminalText { StyledTerminalText::new(text, TerminalStyle::black()) }
pub fn highlight_red(text: &str) -> StyledTerminalText { StyledTerminalText::new(text, TerminalStyle::red()) }
pub fn highlight_green(text: &str) -> StyledTerminalText { StyledTerminalText::new(text, TerminalStyle::green()) }
pub fn highlight_yellow(text: &str) -> StyledTerminalText { StyledTerminalText::new(text, TerminalStyle::yellow()) }
pub fn highlight_blue(text: &str) -> StyledTerminalText { StyledTerminalText::new(text, TerminalStyle::blue()) }
pub fn highlight_magenta(text: &str) -> StyledTerminalText { StyledTerminalText::new(text, TerminalStyle::magenta()) }
pub fn highlight_cyan(text: &str) -> StyledTerminalText { StyledTerminalText::new(text, TerminalStyle::cyan()) }
pub fn highlight_white(text: &str) -> StyledTerminalText { StyledTerminalText::new(text, TerminalStyle::white()) }

// Bright and Bold
pub fn highlight_bright_black(text: &str) -> StyledTerminalText { StyledTerminalText::new(text, TerminalStyle::bright_black()) }
pub fn highlight_bright_red(text: &str) -> StyledTerminalText { StyledTerminalText::new(text, TerminalStyle::bright_red()) }
pub fn highlight_bright_green(text: &str) -> StyledTerminalText { StyledTerminalText::new(text, TerminalStyle::bright_green()) }
pub fn highlight_bright_yellow(text: &str) -> StyledTerminalText { StyledTerminalText::new(text, TerminalStyle::bright_yellow()) }
pub fn highlight_bright_blue(text: &str) -> StyledTerminalText { StyledTerminalText::new(text, TerminalStyle::bright_blue()) }
pub fn highlight_bright_magenta(text: &str) -> StyledTerminalText { StyledTerminalText::new(text, TerminalStyle::bright_magenta()) }
pub fn highlight_bright_cyan(text: &str) -> StyledTerminalText { StyledTerminalText::new(text, TerminalStyle::bright_cyan()) }
pub fn highlight_bright_white(text: &str) -> StyledTerminalText { StyledTerminalText::new(text, TerminalStyle::bright_white()) }

// Tests ----------------------------------------------------------------------------------------- /

#[cfg(test)]
mod tests {
    use super::*;
    use crate::terminal_style::TerminalStyle;

    #[test]
    fn test_highlight_function() {
        assert_eq!(highlight("Hello, World!", TerminalStyle::bright_yellow()).output(), "\u{001B}[1;93mHello, World!\u{001B}[0m");
    }
}