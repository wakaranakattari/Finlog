use std::{
    fs,
    path::Path,
};

use crate::core::spending::SpendingItem;

const EXPENSIVE_DATA_FILE: &str = "./web/public/data/expenses/expensive.json";

/// Serializes and writes all spending items to the JSON data file.
///
/// Creates parent directories if they don't exist.
///
/// # Errors
/// Returns a [`String`] error message if serialization or file writing fails.
pub fn save_items(items: &[SpendingItem]) -> Result<(), String> {
    let json = serde_json::to_string_pretty(items).map_err(|e| e.to_string())?;

    if let Some(parent) = Path::new(EXPENSIVE_DATA_FILE).parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    fs::write(EXPENSIVE_DATA_FILE, json).map_err(|e| e.to_string())
}

/// Removes the item at the given index and persists the updated list.
///
/// # Errors
/// Returns `"Index out of range"` if the index is invalid, or a [`String`]
/// error message if saving fails.
pub fn delete_items(items: &mut Vec<SpendingItem>, index: usize) -> Result<(), String> {
    if index >= items.len() {
        return Err("Index out of range".to_string());
    }
    items.remove(index);
    save_items(items)
}

/// Loads spending items from the JSON data file.
///
/// Returns an empty vector if the file does not exist.
///
/// # Errors
/// Returns a [`String`] error message if reading or deserializing the file fails.
pub fn load_items() -> Result<Vec<SpendingItem>, String> {
    if !Path::new(EXPENSIVE_DATA_FILE).exists() {
        return Ok(vec![]);
    }

    let json = fs::read_to_string(EXPENSIVE_DATA_FILE).map_err(|e| e.to_string())?;
    serde_json::from_str(&json).map_err(|e| e.to_string())
}