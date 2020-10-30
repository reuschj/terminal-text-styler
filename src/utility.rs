use crate::{StyledTerminalText, TerminalStyle};

// Highlight utility functions ------------------------------------------------------------------- /

pub fn highlight(text: &str) -> StyledTerminalText {
    highlight_yellow(text)
}

pub fn highlight_with(text: &str, color: TerminalStyle) -> StyledTerminalText {
    StyledTerminalText::from(text, color)
}

pub fn highlight_yellow(text: &str) -> StyledTerminalText {
    StyledTerminalText::from(text, TerminalStyle::yellow())
}

pub fn highlight_blue(text: &str) -> StyledTerminalText {
    StyledTerminalText::from(text, TerminalStyle::blue())
}

pub fn highlight_green(text: &str) -> StyledTerminalText {
    StyledTerminalText::from(text, TerminalStyle::green())
}

pub fn highlight_red(text: &str) -> StyledTerminalText {
    StyledTerminalText::from(text, TerminalStyle::light_red())
}

