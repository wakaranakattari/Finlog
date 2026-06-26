use std::{
    io::{self, Write},
    process,
    thread,
    time::Duration,
};

use chrono::{Datelike, NaiveDate, Timelike};

use crate::{
    storage::config::*,
    utils::*,
};

/// Clears the terminal screen including scrollback history.
#[inline]
pub fn clear_console() {
    print!("\x1B[2J\x1B[3J\x1B[H");
    io::stdout().flush().unwrap();
}

/// Clears the screen, prints a goodbye message, and exits the process.
#[inline]
pub fn exit() {
    clear_console();
    print_header("Goodbye!");
    process::exit(0);
}

/// Prints a time-based greeting for the given user.
///
/// Uses the current local hour to select morning, afternoon, evening, or night.
pub fn time_greeting(name: &str) {
    let now = chrono::Local::now();
    let hour = now.hour();

    let greeting = match hour {
        5..=11  => "Good morning",
        12..=17 => "Good afternoon",
        18..=21 => "Good evening",
        _       => "Good night",
    };

    println!(
        "{}, {}",
        color_print(greeting, Color::Yellow),
        load_name().unwrap_or_else(|_| name.into())
    );
}

/// Prompts the user to enter a username on first launch and saves it.
pub fn prompt_for_username() {
    let mut username = String::new();

    print!("{}", color_print("First launch! Enter your username: ", Color::Green));
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut username).unwrap();
    save_name(username.trim()).unwrap();
}

/// Prints the main Finlog application header.
#[inline]
pub fn header(title: &str) {
    println!("{}", color_print("┌────────────────────────────────────────┐", Color::Green));
    println!("{}", color_print(&format!("│{:^40}│", title), Color::Yellow));
    println!("{}", color_print("└────────────────────────────────────────┘", Color::Red));
}

/// Prints a centered section header with the given title in a gray box.
#[inline]
pub fn print_header(title: &str) {
    println!("{}", color_print("┌────────────────────────────────────────┐", Color::Gray));
    println!("{}", color_print(&format!("│{:^40}│", title), Color::Gray));
    println!("{}", color_print("└────────────────────────────────────────┘", Color::Gray));
}

/// Waits for the user to press Enter, then clears the console.
///
/// # Errors
/// Returns [`AppError`] if any I/O operation fails.
pub fn back_to_main_menu() -> Result<(), AppError> {
    print!("\n<- Back to main menu");
    io::stdout().flush()?;
    io::stdin().read_line(&mut String::new())?;
    clear_console();
    Ok(())
}

/// Prompts the user for input and returns the trimmed string.
///
/// # Errors
/// Returns [`AppError`] if I/O operations fail.
pub fn prompt_input(prompt: &str) -> Result<String, AppError> {
    print!("{prompt}");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

/// Parses a date string in one of several supported formats.
///
/// Accepted formats: `DD.MM.YYYY`, `DD.MM.YY`, `DD MM YYYY`, `DD MM YY`,
/// `DD-MM-YYYY`, `DD-MM-YY`, `YYYY-MM-DD`.
///
/// Returns `None` if the input doesn't match any format or the year is
/// outside the range 2000–2100.
pub fn parse_date(input: &str) -> Option<NaiveDate> {
    let input = input.trim();

    let formats = [
        "%d.%m.%Y",
        "%d.%m.%y",
        "%d %m %Y",
        "%d %m %y",
        "%d-%m-%Y",
        "%d-%m-%y",
        "%Y-%m-%d",
    ];

    for format in formats {
        if let Ok(date) = NaiveDate::parse_from_str(input, format) {
            let year = date.year();
            if (2000..=2100).contains(&year) {
                return Some(date);
            }
        }
    }

    None
}

/// Pauses execution for 1500 milliseconds.
#[inline]
pub fn thread_sleep_timer() {
    thread::sleep(Duration::from_millis(1500))
}