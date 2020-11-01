# Terminal Text Styler

If you are building a command-line tool with Rust, you may find it useful to highlight output in a style to make it stand out. Doing so, simply means wrapping your text in the necessary [ANSI escape codes](https://en.wikipedia.org/wiki/ANSI_escape_code).

ANSI Color | Normal | Bold | With Background
---- | ---- | ----  | ----
Black | 0;30 | 1;30 | 0;40
Red | 0;31 | 1;31 | 0;41
Green | 0;32 | 1;31 | 0;42
Yellow | 0;33 | 1;31 | 0;43
Blue | 0;34 | 1;31 | 0;44
Magenta | 0;35 | 1;31 | 0;45
Cyan | 0;36 | 1;31 | 0;46
White | 0;37 | 1;31 | 0;47
Bright Black | 0;90 | 1;31 | 0;100
Bright Red | 0;91 | 1;31 | 0;101
Bright Green | 0;92 | 1;31 | 0;102
Bright Yellow | 0;93 | 1;31 | 0;103
Bright Blue | 0;94 | 1;31 | 0;104
Bright Purple | 0;95 | 1;95 | 0;105
Bright Cyan | 0;96 | 1;96 | 0;106
Bright White | 0;97 | 1;97 | 0;107

For example:

```
RED='\033[0;31m'
NC='\033[0m' # No Color
printf "I ${RED}love${NC} Stack Overflow\n"
```
Source: [StackOverflow](https://stackoverflow.com/a/5947802/3055803)

In Rust, we can just replace `'\033'` with the unicode `"\u{001B}"` (Source: [StackOverflow](https://stackoverflow.com/q/40583721/3055803)).

So, we can switch to the desired color by placing the escape code to start that color in our text and end it by using the escape code for no color. That's all there is to it! So, to highlight our text in yellow, we can just do this:

```rust
let greeting = r"\u{001B}[0;31mHello, World!\u{001B}[0m";
println!("{}", greeting);
```

But, that's kinda clunky and hard to read. That's where this package comes in.

## TerminalStyle

With the `TerminalStyle` struct, we can build a color by giving the ANSI escape codes or building out styles with the supplied enums.

```rust
use terminal_text_styler::{TerminalStyle, SGREffect, ANSIForegroundColor};

// Manual with codes
let yellow_manual = TerminalStyle::from(vec![0, 93]);

// Using enums
let yellow = TerminalStyle::new(vec![SGREffect::Normal], Some(ANSIForegroundColor::BrightYellow), None);
let no_color = TerminalStyle::new_empty(); // Initialize without input parameters for no color
assert_eq!(yellow.command(), "\u{001B}[0;93m");
assert_eq!(no_color.command(), "\u{001B}[0m");
assert_eq!(yellow.wrap("Hello, World!"), "\u{001B}[0;93mHello, World!\u{001B}[0m");
```

Now, when placed in a string (or by accessing the `command` method), a `TerminalStyle` instance will generate the ANSI escape code string:

```rust
println!("{}Hello, World!{}", yellow, no_color));
```

This is much better, but we can make it a bit easier. We can call the `wrap` method to wrap the given text in that color.

```rust
println!("{} Blah blah blah...", yellow.wrap("Hello, World!"));
```

### Presets

For your convenience, `TerminalStyle` comes in with some static presets:

```rust
let yellow = TerminalStyle::bright_yellow();
let red = TerminalStyle::red();
// Etc...
```

## StyledTerminalText

The `StyledTerminalText` struct builds on the `wrap` command to put the emphasis back on your text, not the color. The `StyledTerminalText` instance can then be placed right into a `String` (or turned into a `String` by accessing the `output` property):

```rust
use terminal_text_styler::{StyledTerminalText, TerminalStyle};

let greeting = StyledTerminalText::new("Hello, World!", TerminalStyle::bright_yellow());
assert_eq!(greeting.output(), "\u{001B}[1;93mHello, World!\u{001B}[0m");
```

## Highlight Function(s)

The last step to making your code clean and easy is by just using the `highlight` convenience function to generate a `StyledTerminalText`:

```rust
use terminal_text_styler::{highlight, TerminalStyle};

println!("{} Blah blah blah...", highlight("Hello, World!", TerminalStyle::bright_yellow()));
```

Each style in the table above also has a unique highlight function:

```rust
use terminal_text_styler::{highlight_bright_yellow, highlight_red};
println!("This is {} and this is {}.", highlight_bright_yellow("highlighted in bright yellow"), highlight_red("highlighted in red"));
```
