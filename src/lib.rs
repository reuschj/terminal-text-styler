//! # Terminal Text Styler
//!
//! More soon!

// Public exports -------------------------------------------------------------------------------- /

pub use terminal_style::TerminalStyle;
pub use styled_terminal_text::StyledTerminalText;
pub use enums::ansi_foreground::ANSIForegroundColor;
pub use enums::ansi_background::ANSIBackgroundColor;
pub use enums::srg_effect::SGREffect;

// Modules --------------------------------------------------------------------------------------- /

mod terminal_style;
mod styled_terminal_text;
mod enums;
mod traits;
mod utility;