use chrono::Local;
use serde::{Deserialize, Serialize};

use crate::{
    storage,
    utils::{AppError, color::*, console::*},
};

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
        clear_console();
        print_header("Add Spending Item");

        let name = prompt_input("Enter a spending name: ")?;
        let category = prompt_input("Enter a spending category: ")?;
        let amount = prompt_input("Enter a spending amount: ")?
            .parse()
            .unwrap_or(0.0);
        let date = prompt_input("Enter a spending date (DD-MM-YYYY): ")?;

        let date = parse_date(&date)
            .unwrap_or_else(|| Local::now().date_naive())
            .format("%d.%m.%Y")
            .to_string();

        let item = SpendingItem {
            name,
            category,
            amount,
            date,
        };

        println!(
            "{}",
            color_info_print(&format!(
                "Spending item added: {} - {} - {} - {}",
                item.name, item.category, item.amount, item.date
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
        let name_width = self
            .spending
            .iter()
            .map(|i| i.name.chars().count())
            .max()
            .unwrap_or(0);
        let cat_width = self
            .spending
            .iter()
            .map(|i| i.category.chars().count())
            .max()
            .unwrap_or(0);
        let amount_width = self
            .spending
            .iter()
            .map(|i| format!("{:.2}", i.amount).len())
            .max()
            .unwrap_or(0);
        let date_width = self
            .spending
            .iter()
            .map(|i| i.date.chars().count())
            .max()
            .unwrap_or(0);

        for (index, item) in self.spending.iter().enumerate() {
            let idx = format!("{}.", index + 1);
            let name = format!(
                "{}{}",
                item.name,
                " ".repeat(name_width - item.name.chars().count())
            );
            let cat = format!(
                "{}{}",
                item.category,
                " ".repeat(cat_width - item.category.chars().count())
            );
            let amount = format!("{:.2}", item.amount);
            let date = format!(
                "{}{}",
                item.date,
                " ".repeat(date_width - item.date.chars().count())
            );

            println!(
                "{:<idx_width$} {} | {} | {:>amount_width$} | {}",
                idx,
                name,
                cat,
                amount,
                date,
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
                item.date
            );
        }

        let input = prompt_input("Enter number of item to delete: ")?;
        let index: usize = input.parse().unwrap_or(0) - 1;

        if index < self.spending.len() {
            storage::delete_items(&mut self.spending, index)?;
            println!("{}", color_info_print("Item deleted successfully."));
            thread_sleep_timer();
            self.view_spending_items()?;
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
            "\n{}{:.2}\n{} {}",
            color_print("Total spending: ", Color::Cyan),
            total,
            color_print("Total records: ", Color::Cyan),
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
        self.spending
            .iter()
            .filter(|item| item.name.to_lowercase().contains(&query))
            .collect()
    }

    pub fn handle_find_item_by_name(&self) -> Result<(), AppError> {
        clear_console();
        print_header("Find Item by Name");

        let input = prompt_input("Enter item name: ")?;

        let results = self.find_item_by_name(&input);
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
        Ok(())
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
