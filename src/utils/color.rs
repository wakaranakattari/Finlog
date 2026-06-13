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
}

pub enum BackgroundColor {
    BackgroundReset,
    BackgroundRed,
    BackgroundGreen,
    BackgroundYellow,
    BackgroundBlue,
    BackgroundMagenta,
    BackgroundCyan,
    BackgroundWhite,
}

impl Color {
    /// Returns ANSI escape code for the color
    pub fn color_text(&self) -> &str {
        match self {
            Color::Reset => "\x1B[0m",
            Color::Black => "\x1B[30m",
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

impl BackgroundColor {
    /// Returns ANSI escape code for the background color
    pub fn color_text(&self) -> &str {
        match self {
            BackgroundColor::BackgroundReset => "\x1B[49m",
            BackgroundColor::BackgroundRed => "\x1B[41m",
            BackgroundColor::BackgroundGreen => "\x1B[42m",
            BackgroundColor::BackgroundYellow => "\x1B[43m",
            BackgroundColor::BackgroundBlue => "\x1B[44m",
            BackgroundColor::BackgroundMagenta => "\x1B[45m",
            BackgroundColor::BackgroundCyan => "\x1B[46m",
            BackgroundColor::BackgroundWhite => "\x1B[47m",
        }
    }
}

/// Formats text with color and without newline, resets after
pub fn color_print(text: &str, color: Color) -> String {
    format!(
        "{}{}{}",
        color.color_text(),
        text,
        Color::Reset.color_text()
    )
}

/// Formats text with color and newline, resets after
pub fn color_println(text: &str, color: Color) -> String {
    format!(
        "{}{}{}\n",
        color.color_text(),
        text,
        Color::Reset.color_text()
    )
}

/// Formats warning message with yellow color
pub fn color_warning_print(text: &str) -> String {
    format!(
        "{} WARNING {}: {}{}{}\n",
        BackgroundColor::BackgroundYellow.color_text(),
        BackgroundColor::BackgroundReset.color_text(),
        Color::Yellow.color_text(),
        text,
        Color::Reset.color_text()
    )
}

/// Formats error message with red color
pub fn color_error_print(text: &str) -> String {
    format!(
        "{} ERROR {}: {}{}{}\n",
        BackgroundColor::BackgroundRed.color_text(),
        BackgroundColor::BackgroundReset.color_text(),
        Color::Red.color_text(),
        text,
        Color::Reset.color_text()
    )
}

/// Formats success message with green color
pub fn color_success_print(text: &str) -> String {
    format!(
        "{} SUCCESS {}: {}{}{}\n",
        BackgroundColor::BackgroundGreen.color_text(),
        BackgroundColor::BackgroundReset.color_text(),
        Color::Green.color_text(),
        text,
        Color::Reset.color_text()
    )
}

/// Formats info message with blue color
pub fn color_info_print(text: &str) -> String {
    format!(
        "{} INFO {}: {}{}{}\n",
        BackgroundColor::BackgroundBlue.color_text(),
        BackgroundColor::BackgroundReset.color_text(),
        Color::Blue.color_text(),
        text,
        Color::Reset.color_text()
    )
}
