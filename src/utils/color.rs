pub enum Color {
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
    /// Returns ANSI escape code for the color
    pub fn code(&self) -> &str {
        match self {
            Color::Reset => "\x1B[0m",
            Color::Red => "\x1B[31m",
            Color::Green => "\x1B[32m",
            Color::Yellow => "\x1B[33m",
            Color::Blue => "\x1B[34m",
            Color::Magenta => "\x1B[35m",
            Color::Cyan => "\x1B[36m",
            Color::White => "\x1B[37m",
        }
    }
}

/// Formats text with color and newline, resets after
pub fn color_println(text: &str, color: Color) -> String {
    format!("{}{}{}\n", color.code(), text, Color::Reset.code())
}

/// Formats warning message with yellow color
pub fn color_warning_print(text: &str) -> String {
    format!(
        "[WARNING]: {}{}{}\n",
        Color::Yellow.code(),
        text,
        Color::Reset.code()
    )
}

/// Formats error message with red color
pub fn color_error_print(text: &str) -> String {
    format!(
        "[ERROR]: {}{}{}\n",
        Color::Red.code(),
        text,
        Color::Reset.code()
    )
}

/// Formats success message with green color
pub fn color_success_print(text: &str) -> String {
    format!(
        "[SUCCESS]: {}{}{}\n",
        Color::Green.code(),
        text,
        Color::Reset.code()
    )
}

/// Formats info message with blue color
pub fn color_info_print(text: &str) -> String {
    format!(
        "[INFO]: {}{}{}\n",
        Color::Blue.code(),
        text,
        Color::Reset.code()
    )
}
