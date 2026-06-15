use crate::core::spending::SpendingItem;
use std::path::PathBuf;

const DATA_FILE: &str = "data/expenses/expensive.json";

pub fn save_items(items: &[SpendingItem]) -> Result<(), String> {
    let json = serde_json::to_string_pretty(items).unwrap();

    let path = PathBuf::from(DATA_FILE);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    std::fs::write(DATA_FILE, json).map_err(|e| e.to_string())
}

pub fn load_items() -> Result<Vec<SpendingItem>, String> {
    if !PathBuf::from(DATA_FILE).exists() {
        return Ok(vec![]);
    }

    let json = std::fs::read_to_string(DATA_FILE).map_err(|e| e.to_string())?;
    serde_json::from_str(&json).map_err(|e| e.to_string())
}
