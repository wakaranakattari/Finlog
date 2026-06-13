use chrono::Local;
use finlog::utils::*;
use std::io::{self, Write};
use std::process;
use std::thread;
use std::time::Duration;

// ================================ //
//            UTILITIES             //
// ================================ //

/// Clears terminal including scrollback history
fn clear_console() {
    print!("\x1B[2J\x1B[3J\x1B[H");
    io::stdout().flush().unwrap();
}

/// Clears screen and exits with Magenta message
fn exit() {
    clear_console();
    println!("{}", color_print("Goodbye!", Color::Magenta));
    process::exit(0);
}

fn header() {
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

// ================================ //
//             CORE LOGIC           //
// ================================ //

/// Adds a spending item to the vector
fn spending_item(spending: &mut Vec<String>) {
    let mut item = String::new();

    clear_console();

    print!("Enter a spending item: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut item)
        .expect(&color_error_print("Error reading line"));

    spending.push(item.trim().to_string());

    print!(
        "{}",
        color_info_print(&format!(
            "Spending item added: {}",
            spending[spending.len() - 1]
        ))
    );
    thread::sleep(Duration::from_millis(1500));
    clear_console();
}

/// Displays all spending items
fn view_spending_items(spending: &[String]) {
    clear_console();
    println!("Your spending items:");
    for (index, item) in spending.iter().enumerate() {
        println!("{}. {}", index + 1, item);
    }

    print!("\n<- Back to main menu");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut String::new()).unwrap();
    // thread::sleep(Duration::from_millis(1500));
    clear_console();
}

/// Main menu handler
fn menu(spending: &mut Vec<String>) {
    let now = Local::now();

    header();

    println!(
        "{}{}",
        color_print("Time: ", Color::Cyan),
        now.format("%H:%M:%S")
    );

    println!(
        "{}{}",
        color_print("Date today: ", Color::Cyan),
        now.format("%d.%m.%Y")
    );

    println!("\n1. Add spending item");
    println!("2. View spending items");
    println!("3. Exit");

    print!("\n> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect(&color_error_print("Error read line"));

    match input.trim() {
        "1" => spending_item(spending),
        "2" => view_spending_items(spending),
        "3" => exit(),
        _ => println!("{}", color_error_print("Invalid choice")),
    }
}

// ================================ //
//              ENTRY               //
// ================================ //

fn main() {
    let mut spending: Vec<String> = Vec::new();

    clear_console();

    loop {
        menu(&mut spending);
    }
}
