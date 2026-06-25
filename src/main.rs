use chrono::Local;
use finlog::storage::config::config::check_name;
use finlog::storage::config::load_name;
use finlog::utils::*;
use finlog::{core::SpendingManager, server::run_server};
use std::io::{self, Write};

/// Renders and handles the main menu loop iteration.
///
/// Displays the current time, date, greeting, and menu options.
/// Reads user input and dispatches to the appropriate handler.
///
/// # Errors
/// Returns [`AppError`] if any I/O operation fails.
async fn menu(spending: &mut SpendingManager) -> Result<(), AppError> {
    let now = Local::now();

    header();

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

    let username = load_name().unwrap_or("Name".to_string());
    time_greeting(&username);

    let items = [
        "Start WEB Version",
        "Add spending item",
        "View spending items",
        "Show statistics",
        "Find item by name",
        "Delete item",
        "Exit",
    ];

    println!("┌────────────────────────────────────────┐");
    for (i, item) in items.iter().enumerate() {
        println!("│ {}. {:<35} │", i + 1, item);
    }
    println!("└────────────────────────────────────────┘");

    print!("\n> ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    match input.trim() {
        "1" => {
            let _ = run_server(3000).await;
            back_to_main_menu()?;
        }
        "2" => spending.add_spending_item()?,
        "3" => spending.view_spending_items()?,
        "4" => spending.show_spending_statistic()?,
        "5" => {
            clear_console();
            print_header("Find Item by Name");

            print!("{}", color_print("Enter item name: ", Color::Green));
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            let results = spending.find_item_by_name(input.trim());
            if results.is_empty() {
                println!("{}", color_error_print("Item not found."));
            } else {
                for item in results {
                    println!(
                        "{} | {} | {:.2} | {}",
                        item.name, item.category, item.amount, item.date
                    );
                }
            }

            back_to_main_menu()?;
        }
        "6" => spending.delete_spending_item()?,
        "7" => exit(),
        _ => {
            clear_console();
            println!("{}", color_error_print("Coming soon"));
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