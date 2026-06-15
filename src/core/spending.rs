use crate::storage;
use crate::utils::color::*;
use crate::utils::console::*;
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpendingItem {
    pub name: String,
    pub category: String,
    pub amount: f64,
    pub date: String,
}

pub struct SpendingManager {
    spending: Vec<SpendingItem>,
}

impl SpendingManager {
    pub fn new() -> Self {
        let items = storage::load_items().unwrap_or_default();
        Self { spending: items }
    }

    pub fn save_to_file(&self) {
        storage::save_items(&self.spending).unwrap();
    }

    pub fn add_spending_item(&mut self) {
        let mut name = String::new();
        let mut category = String::new();
        let mut amount = String::new();
        let mut date = String::new();

        prompt_user_input("Enter a spending name", &mut name);

        prompt_user_input("Enter a spending category", &mut category);

        prompt_user_input("Enter a spending amount", &mut amount);
        let amount: f64 = amount.trim().parse().unwrap_or(0.0);

        prompt_user_input("Enter a spending date (YYYY-MM-DD)", &mut date);
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
        storage::save_items(&self.spending).unwrap();

        thread::sleep(Duration::from_millis(1500));
        clear_console();
    }

    pub fn view_spending_items(&self) {
        if self.spending.is_empty() {
            println!("{}", color_info_print("No spending items to display."));
            return;
        }

        for (index, item) in self.spending.iter().enumerate() {
            println!(
                "{}. {} | {} | ${:.2} | {}",
                index + 1,
                item.name,
                item.category,
                item.amount,
                item.date,
            );
        }

        print!("\n<- Back to main menu");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut String::new()).unwrap();
    }

    pub fn get_items(&self) -> &Vec<SpendingItem> {
        &self.spending
    }
}

impl Default for SpendingManager {
    fn default() -> Self {
        Self::new()
    }
}
