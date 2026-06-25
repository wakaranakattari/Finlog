use crate::utils::color::*;
use chrono::Datelike;
use chrono::NaiveDate;
use std::io::{self, Write};
use std::process;

// ================================ //
//            UTILITIES             //
// ================================ //

/// Clears terminal including scrollback history
pub fn clear_console() {
    print!("\x1B[2J\x1B[3J\x1B[H");
    io::stdout().flush().unwrap();
}

/// Clears screen and exits with Magenta message
pub fn exit() {
    clear_console();
    println!("{}", color_print("Goodbye!", Color::Magenta));
    process::exit(0);
}

pub fn header() {
    println!(
        "{}",
        color_print("┌────────────────────────────────────────┐", Color::Green)
    );
    println!(
        "{}",
        color_print("│                 Finlog                 │", Color::Yellow)
    );
    println!(
        "{}",
        color_print("└────────────────────────────────────────┘", Color::Red)
    );
}

pub fn add_spending_header() {
    println!(
        "{}",
        color_print("┌────────────────────────────────────────┐", Color::Gray)
    );
    println!(
        "{}",
        color_print("│               Add Spending             │", Color::Gray)
    );
    println!(
        "{}",
        color_print("└────────────────────────────────────────┘", Color::Gray)
    );
}

pub fn view_spending_header() {
    println!(
        "{}",
        color_print("┌────────────────────────────────────────┐", Color::Gray)
    );
    println!(
        "{}",
        color_print("│              View Spending             │", Color::Gray)
    );
    println!(
        "{}",
        color_print("└────────────────────────────────────────┘", Color::Gray)
    );
}

pub fn prompt_user_input(prompt: &str, input: &mut String) {
    print!("{prompt}: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(input)
        .expect(&color_error_print("Error reading line"));
}

pub fn parse_date(input: &str) -> Option<NaiveDate> {
    let input = input.trim();

    let formats = [
        "%d.%m.%Y", // 20.05.2026
        "%d.%m.%y", // 20.05.26
        "%d %m %Y", // 20 05 2026
        "%d %m %y", // 20 05 26
        "%d-%m-%Y", // 20-05-2026
        "%d-%m-%y", // 20-05-26
        "%Y-%m-%d", // 2026-05-20
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
