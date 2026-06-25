use crate::storage;
use crate::utils::color::*;
use crate::utils::console::*;
use crate::utils::AppError;
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

/// Represents a single spending record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpendingItem {
    pub name: String,
    pub category: String,
    pub amount: f64,
    pub date: String,
}

/// Manages the in-memory list of spending items and persists them to storage.
pub struct SpendingManager {
    spending: Vec<SpendingItem>,
}

impl SpendingManager {
    /// Loads spending items from storage on startup.
    pub fn new() -> Self {
        let items = storage::load_items().unwrap_or_default();
        Self { spending: items }
    }

    /// Interactively prompts the user for a new spending item and saves it.
    ///
    /// # Errors
    /// Returns [`AppError`] if any I/O or storage operation fails.
    pub fn add_spending_item(&mut self) -> Result<(), AppError> {
        let mut name = String::new();
        let mut category = String::new();
        let mut amount = String::new();
        let mut date = String::new();

        clear_console();
        print_header("Add Spending");
        prompt_user_input("Enter a spending name", &mut name)?;

        clear_console();
        print_header("Add Spending");
        prompt_user_input("Enter a spending category", &mut category)?;

        clear_console();
        print_header("Add Spending");
        prompt_user_input("Enter a spending amount", &mut amount)?;
        let amount: f64 = amount.trim().parse().unwrap_or(0.0);

        clear_console();
        print_header("Add Spending");
        prompt_user_input("Enter a spending date (YYYY-MM-DD)", &mut date)?;
        let date = parse_date(&date)
            .unwrap_or_else(|| Local::now().date_naive())
            .format("%d.%m.%Y")
            .to_string();

        let item = SpendingItem {
            name: name.trim().to_string(),
            category: category.trim().to_string(),
            amount,
            date,
        };

        println!(
            "{}",
            color_info_print(&format!(
                "Spending item added: {} - {} - {} - {}",
                name.trim(),
                category.trim(),
                amount,
                item.date
            ))
        );

        self.spending.push(item);
        storage::save_items(&self.spending)?;

        thread_sleep_timer();
        clear_console();
        Ok(())
    }

    /// Displays all spending items in an aligned table.
    ///
    /// Column widths are calculated dynamically to handle Unicode correctly.
    ///
    /// # Errors
    /// Returns [`AppError`] if any I/O operation fails.
    pub fn view_spending_items(&self) -> Result<(), AppError> {
        clear_console();
        print_header("View Spending");

        if self.spending.is_empty() {
            println!("{}", color_info_print("No spending items to display."));
            return Ok(());
        }

        let idx_width = self.spending.len().to_string().len() + 1;
        let name_width = self.spending.iter().map(|i| i.name.chars().count()).max().unwrap_or(0);
        let cat_width = self.spending.iter().map(|i| i.category.chars().count()).max().unwrap_or(0);
        let amount_width = self.spending.iter()
            .map(|i| format!("{:.2}", i.amount).len())
            .max().unwrap_or(0);
        let date_width = self.spending.iter().map(|i| i.date.chars().count()).max().unwrap_or(0);

        for (index, item) in self.spending.iter().enumerate() {
            let idx = format!("{}.", index + 1);
            let name = format!("{}{}", item.name, " ".repeat(name_width - item.name.chars().count()));
            let cat = format!("{}{}", item.category, " ".repeat(cat_width - item.category.chars().count()));
            let amount = format!("{:.2}", item.amount);
            let date = format!("{}{}", item.date, " ".repeat(date_width - item.date.chars().count()));

            println!(
                "{:<idx_width$} {} | {} | {:>amount_width$} | {}",
                idx, name, cat, amount, date,
                idx_width = idx_width,
            );
        }

        back_to_main_menu()?;
        Ok(())
    }

    /// Prompts the user to select an item by number and deletes it from storage.
    ///
    /// After successful deletion, re-renders the updated list.
    ///
    /// # Errors
    /// Returns [`AppError`] if any I/O operation fails.
    pub fn delete_spending_item(&mut self) -> Result<(), AppError> {
        clear_console();
        print_header("Delete Spending Item");

        if self.spending.is_empty() {
            println!("{}", color_info_print("No spending items to display."));
            return Ok(());
        }

        for (index, item) in self.spending.iter().enumerate() {
            println!(
                "{}. {} | {} | {:.2} | {}",
                index + 1,
                item.name,
                item.category,
                item.amount,
                item.date,
            );
        }

        print!("{}", color_print("Enter number of item to delete: ", Color::Green));
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let index: usize = input.trim().parse().unwrap_or(0) - 1;

        if index < self.spending.len() {
            if let Err(e) = storage::delete_items(&mut self.spending, index) {
                println!("{}", color_error_print(&format!("Error deleting item: {}", e)));
            } else {
                println!("{}", color_info_print("Item deleted successfully."));
                thread_sleep_timer();
                self.view_spending_items()?;
            }
        } else {
            println!("{}", color_error_print("Invalid item number."));
        }

        Ok(())
    }

    /// Displays total spending amount and record count across all items.
    ///
    /// # Errors
    /// Returns [`AppError`] if any I/O operation fails.
    pub fn show_spending_statistic(&self) -> Result<(), AppError> {
        clear_console();
        print_header("Spending Statistics");

        if self.spending.is_empty() {
            println!("{}", color_info_print("No spending items to display."));
            return Ok(());
        }

        let total: f64 = self.spending.iter().map(|i| i.amount).sum();
        let count = self.spending.len();

        print!(
            "\n{}{:.2}",
            color_print("Total spending: ", Color::Cyan),
            total
        );

        io::stdout().flush()?;

        println!(
            "{}{}",
            color_print("\nTotal records: ", Color::Cyan),
            count
        );

        back_to_main_menu()?;
        Ok(())
    }

    /// Searches for spending items whose name contains the given query.
    ///
    /// The search is case-insensitive and supports Unicode via [`str::to_lowercase`].
    pub fn find_item_by_name(&self, name: &str) -> Vec<&SpendingItem> {
        let query = name.to_lowercase();
        self.spending.iter()
            .filter(|item| item.name.to_lowercase().contains(&query))
            .collect()
    }

    /// Returns a slice of all spending items.
    pub fn get_items(&self) -> &[SpendingItem] {
        &self.spending
    }
}

impl Default for SpendingManager {
    fn default() -> Self {
        Self::new()
    }
}