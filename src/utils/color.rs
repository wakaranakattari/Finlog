/// Foreground color for terminal output.
pub enum Color {
    Reset,
    Red,
    Green,
    Yellow,
    Blue,
    Black,
    Magenta,
    Cyan,
    White,
    Gray,
}

/// Background color for terminal output.
pub enum BackgroundColor {
    Reset,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl Color {
    /// Returns the ANSI escape code for this foreground color.
    pub fn color_text(&self) -> &str {
        match self {
            Color::Reset   => "\x1B[0m",
            Color::Black   => "\x1B[30m",
            Color::Red     => "\x1B[31m",
            Color::Green   => "\x1B[32m",
            Color::Yellow  => "\x1B[33m",
            Color::Blue    => "\x1B[34m",
            Color::Magenta => "\x1B[35m",
            Color::Cyan    => "\x1B[36m",
            Color::White   => "\x1B[37m",
            Color::Gray    => "\x1B[90m",
        }
    }
}

impl BackgroundColor {
    /// Returns the ANSI escape code for this background color.
    pub fn color_text(&self) -> &str {
        match self {
            BackgroundColor::Reset   => "\x1B[49m",
            BackgroundColor::Red     => "\x1B[41m",
            BackgroundColor::Green   => "\x1B[42m",
            BackgroundColor::Yellow  => "\x1B[43m",
            BackgroundColor::Blue    => "\x1B[44m",
            BackgroundColor::Magenta => "\x1B[45m",
            BackgroundColor::Cyan    => "\x1B[46m",
            BackgroundColor::White   => "\x1B[47m",
        }
    }
}

/// Wraps `text` in the given foreground color, resetting after.
pub fn color_print(text: &str, color: Color) -> String {
    format!("{}{}{}", color.color_text(), text, Color::Reset.color_text())
}

/// Wraps `text` in the given foreground color with a trailing newline, resetting after.
pub fn color_println(text: &str, color: Color) -> String {
    format!("{}{}{}\n", color.color_text(), text, Color::Reset.color_text())
}

/// Formats `text` as a yellow `WARNING` label followed by yellow message text.
pub fn color_warning_print(text: &str) -> String {
    format!(
        "{} WARNING {}: {}{}{}\n",
        BackgroundColor::Yellow.color_text(),
        BackgroundColor::Reset.color_text(),
        Color::Yellow.color_text(),
        text,
        Color::Reset.color_text()
    )
}

/// Formats `text` as a red `ERROR` label followed by red message text.
pub fn color_error_print(text: &str) -> String {
    format!(
        "{} ERROR {}: {}{}{}\n",
        BackgroundColor::Red.color_text(),
        BackgroundColor::Reset.color_text(),
        Color::Red.color_text(),
        text,
        Color::Reset.color_text()
    )
}

/// Formats `text` as a green `SUCCESS` label followed by green message text.
pub fn color_success_print(text: &str) -> String {
    format!(
        "{} SUCCESS {}: {}{}{}\n",
        BackgroundColor::Green.color_text(),
        BackgroundColor::Reset.color_text(),
        Color::Green.color_text(),
        text,
        Color::Reset.color_text()
    )
}

/// Formats `text` as a blue `INFO` label followed by blue message text.
pub fn color_info_print(text: &str) -> String {
    format!(
        "{} INFO {}: {}{}{}\n",
        BackgroundColor::Blue.color_text(),
        BackgroundColor::Reset.color_text(),
        Color::Blue.color_text(),
        text,
        Color::Reset.color_text()
    )
}