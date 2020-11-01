//! # Terminal Text Styler
//!
//! If you are building a command-line tool with Rust, you may find it useful to highlight output
//! in a style to make it stand out. Doing so, simply means wrapping your text in the necessary
//! [ANSI escape codes](https://en.wikipedia.org/wiki/ANSI_escape_code).

// Public exports -------------------------------------------------------------------------------- /

pub use terminal_style::TerminalStyle;
pub use styled_terminal_text::StyledTerminalText;
// Enums
pub use enums::ansi_foreground::ANSIForegroundColor;
pub use enums::ansi_background::ANSIBackgroundColor;
pub use enums::srg_effect::SGREffect;
// Utility
pub use utility::*;
// Traits
pub use traits::Coded;

// Modules --------------------------------------------------------------------------------------- /

mod terminal_style;
mod styled_terminal_text;
pub mod enums;
pub mod traits;
pub mod utility;