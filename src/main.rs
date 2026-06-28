use std::io::{self, Write};

use chrono::Local;

use finlog::{
    core::SpendingManager,
    server::run_server,
    storage::config::config::{check_name, load_name},
    utils::*,
};

const WEB_PORT: u16 = 3000;

const MENU_ITEMS: [&str; 7] = [
    "Start WEB Version",
    "Add spending item",
    "View spending items",
    "Show statistics",
    "Find item by name",
    "Delete item",
    "Exit",
];

/// Renders and handles the main menu loop iteration.
///
/// Displays the current time, date, greeting, and menu options.
/// Reads user input and dispatches to the appropriate handler.
///
/// # Errors
/// Returns [`AppError`] if any I/O operation fails.
async fn menu(spending: &mut SpendingManager) -> Result<(), AppError> {
    let now = Local::now();

    header("Finlog");

    println!(
        "\n{}{}",
        color_print("Time: ", Color::Cyan),
        now.format("%H:%M")
    );

    println!(
        "{}{}",
        color_print("Date today: ", Color::Cyan),
        now.format("%d.%m.%Y")
    );

    let username = load_name().unwrap_or_else(|_| "Name".to_string());
    time_greeting(&username);
    println!();

    println!("┌────────────────────────────────────────┐");
    for (i, item) in MENU_ITEMS.iter().enumerate() {
        println!("│ {}. {:<35} │", i + 1, item);
    }
    println!("└────────────────────────────────────────┘");

    print!("\n> ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    match input.trim() {
        "1" => {
            let _ = run_server(WEB_PORT).await;
            back_to_main_menu()?;
        }
        "2" => spending.add_spending_item()?,
        "3" => spending.view_spending_items()?,
        "4" => spending.show_spending_statistic()?,
        "5" => spending.handle_find_item_by_name()?,
        "6" => spending.delete_spending_item()?,
        "7" => exit(),
        _ => {
            clear_console();
            println!(
                "{}",
                color_error_print("Invalid input or invalid menu item")
            );
            thread_sleep_timer();
        }
    }

    Ok(())
}

// ┌──────────────────────────────┐ //
// │            ENTRY             │ //
// └──────────────────────────────┘ //

/// Application entry point.
///
/// Initializes the spending manager, checks for a saved username
/// on first launch, then runs the main menu in a loop until the
/// user exits.
#[tokio::main]
async fn main() {
    clear_console();
    let mut spending = SpendingManager::new();

    check_name();

    loop {
        if let Err(e) = menu(&mut spending).await {
            eprintln!("{}", e);
        }
    }
}
