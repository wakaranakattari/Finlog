use chrono::Local;
use finlog::utils::*;
use finlog::{core::SpendingManager, server::run_server};
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

/// Main menu handler
async fn menu(spending: &mut SpendingManager) {
    let now = Local::now();

    header();

    println!(
        "\n{}{}",
        color_print("Time: ", Color::Cyan),
        now.format("%H:%M:%S")
    );

    println!(
        "{}{}",
        color_print("Date today: ", Color::Cyan),
        now.format("%d.%m.%Y")
    );

    println!("{}, Nikita!\n", color_print("Good evening", Color::Yellow));

    println!("┌────────────────────────────────────────┐");
    println!("│ 1. Start WEB Version                   │");
    println!("│ 2. Add spending item                   │");
    println!("│ 3. View spending items                 │");
    println!("│ 4. Calculate percentage of total       │");
    println!("│ 5. Show statistics                     │");
    println!("│ 6. Find item by name                   │");
    println!("│ 7. Delete item                         │");
    println!("│ 8. Settings                            │");
    println!("│ 9. Exit                                │");
    println!("└────────────────────────────────────────┘");

    print!("\n> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect(&color_error_print("Error read line"));

    match input.trim() {
        "1" => {
            tokio::spawn(async {
                run_server(3000).await;
            });

            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

            println!("\n<- Press Enter to back to main menu");
            let mut dummy = String::new();
            io::stdin().read_line(&mut dummy).unwrap();

            clear_console();
        }
        "2" => spending.add_spending_item(),
        "3" => spending.view_spending_items(),
        //  Coming soon methods
        // "4" => spending.calculate_percentage_of_total(),
        // "5" => spending.show_statistics(),
        // "6" => spending.find_item_by_name(),
        // "7" => spending.delete_spending_item(),
        // "8" => spending.settings(),
        "9" => exit(),
        "" => {
            if input.trim().is_empty() {
                return;
            }
        }
        _ => {
            clear_console();
            println!("{}", color_error_print("Coming soon"));
            thread::sleep(Duration::from_millis(1500));
        }
    }
}

// ┌──────────────────────────────┐ //
// │            ENTRY             │ //
// └──────────────────────────────┘ //

#[tokio::main]
async fn main() {
    clear_console();
    let mut spending = SpendingManager::new();

    loop {
        menu(&mut spending).await;
    }
}
