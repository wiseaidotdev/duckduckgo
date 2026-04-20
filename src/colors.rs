// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// An enumeration representing ANSI color codes for text styling.
pub enum AnsiColor {
    /// Cyan color.
    Cyan,
    /// Blue color.
    Blue,
    /// Yellow color.
    Yellow,
    /// Red color.
    Red,
    /// Green color.
    Green,
    /// Magenta color.
    Magenta,
    /// Black color.
    Black,
    /// White color.
    White,
    /// Bright Red color.
    BrightRed,
    /// Bright Green color.
    BrightGreen,
    /// Bright Yellow color.
    BrightYellow,
    /// Bright Blue color.
    BrightBlue,
    /// Bright Magenta color.
    BrightMagenta,
    /// Bright Cyan color.
    BrightCyan,
    /// Dark Gray color.
    DarkGray,
    /// Light Gray color.
    LightGray,
    /// Olive color.
    Olive,
    /// Maroon color.
    Maroon,
    /// Navy color.
    Navy,
    /// Teal color.
    Teal,
    /// Aqua color.
    Aqua,
    /// Purple color.
    Purple,
    /// Silver color.
    Silver,
    /// Dark Red color.
    DarkRed,
    /// Lime color.
    Lime,
    /// Brown color.
    Brown,
    /// Salmon color.
    Salmon,
    /// Sky Blue color.
    SkyBlue,
    /// Gold color.
    Gold,
}

impl AnsiColor {
    /// Returns the ANSI escape code for the associated color.
    ///
    /// # Returns
    /// `&'static str` - The ANSI escape code for the color.
    ///
    /// # Examples
    /// ```
    /// use duckduckgo::colors::AnsiColor;
    ///
    /// let cyan_code = AnsiColor::Cyan.escape_code();
    /// assert_eq!(cyan_code, "\u{001B}[36m");
    /// ```
    pub fn escape_code(&self) -> &'static str {
        match self {
            AnsiColor::Cyan => "\u{001B}[36m",
            AnsiColor::Blue => "\u{001B}[34m",
            AnsiColor::Yellow => "\u{001B}[33m",
            AnsiColor::Red => "\u{001B}[31m",
            AnsiColor::Green => "\u{001B}[32m",
            AnsiColor::Magenta => "\u{001B}[35m",
            AnsiColor::Black => "\u{001B}[30m",
            AnsiColor::White => "\u{001B}[37m",
            AnsiColor::BrightRed => "\u{001B}[91m",
            AnsiColor::BrightGreen => "\u{001B}[92m",
            AnsiColor::BrightYellow => "\u{001B}[93m",
            AnsiColor::BrightBlue => "\u{001B}[94m",
            AnsiColor::BrightMagenta => "\u{001B}[95m",
            AnsiColor::BrightCyan => "\u{001B}[96m",
            AnsiColor::DarkGray => "\u{001B}[90m",
            AnsiColor::LightGray => "\u{001B}[37;1m",
            AnsiColor::Olive => "\u{001B}[33;1m",
            AnsiColor::Maroon => "\u{001B}[31;1m",
            AnsiColor::Navy => "\u{001B}[34;1m",
            AnsiColor::Teal => "\u{001B}[36;1m",
            AnsiColor::Aqua => "\u{001B}[96;1m",
            AnsiColor::Purple => "\u{001B}[35;1m",
            AnsiColor::Silver => "\u{001B}[37;2m",
            AnsiColor::DarkRed => "\u{001B}[31;2m",
            AnsiColor::Lime => "\u{001B}[32;2m",
            AnsiColor::Brown => "\u{001B}[33;2m",
            AnsiColor::Salmon => "\u{001B}[91;1m",
            AnsiColor::SkyBlue => "\u{001B}[94;1m",
            AnsiColor::Gold => "\u{001B}[33;3m",
        }
    }
}

/// A structure representing ANSI text styling.
pub struct AnsiStyle {
    /// A flag indicating whether text should be bold.
    pub bold: bool,
    /// An optional ANSI color for text styling.
    pub color: Option<AnsiColor>,
}

impl AnsiStyle {
    /// Returns the ANSI escape code for the associated text style.
    ///
    /// # Returns
    /// `String` - The ANSI escape code for the text style.
    ///
    /// # Examples
    /// ```
    /// use duckduckgo::colors::{AnsiColor, AnsiStyle};
    ///
    /// let style = AnsiStyle { bold: true, color: Some(AnsiColor::Cyan) };
    /// let escape_code = style.escape_code();
    /// assert_eq!(escape_code, "\u{001B}[1m\u{001B}[36m");
    /// ```
    pub fn escape_code(&self) -> String {
        let mut code = String::new();

        if self.bold {
            code.push_str("\u{001B}[1m");
        }

        if let Some(color) = &self.color {
            code.push_str(color.escape_code());
        }

        code
    }

    /// Returns the ANSI escape code for resetting text styles.
    ///
    /// # Returns
    /// `&'static str` - The ANSI escape code for resetting text styles.
    ///
    /// # Examples
    /// ```
    /// use duckduckgo::colors::AnsiStyle;
    ///
    /// let reset_code = AnsiStyle::reset_code();
    /// assert_eq!(reset_code, "\u{001B}[0m");
    /// ```
    pub fn reset_code() -> &'static str {
        "\u{001B}[0m"
    }
}

// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
