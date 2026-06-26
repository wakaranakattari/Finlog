use std::{
    fs,
    path::Path
};

use serde::{Deserialize, Serialize};

use crate::utils::{clear_console, prompt_for_username};

const CONFIG_DATA_FILE: &str = "./web/public/data/config/config.json";
const DEFAULT_NAME: &str = "User";

/// Stored user configuration.
#[derive(Serialize, Deserialize)]
struct Config {
    name: String,
}

/// Checks if a username is configured and prompts for one if not.
///
/// Called once on startup. If the config file is missing or empty,
/// the user is asked to enter their name before continuing.
pub fn check_name() {
    let has_valid_config = Path::new(CONFIG_DATA_FILE)
        .exists()
        .then(|| fs::metadata(CONFIG_DATA_FILE).ok())
        .flatten()
        .map(|m| m.len() > 0)
        .unwrap_or(false);

    if !has_valid_config {
        prompt_for_username();
        clear_console();
    }
}

/// Saves the given username to the config file.
///
/// Creates parent directories if they don't exist.
///
/// # Errors
/// Returns a [`String`] error message if serialization or file writing fails.
pub fn save_name(name: &str) -> Result<(), String> {
    let config = Config {
        name: name.to_string(),
    };
    let json = serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?;
    if let Some(parent) = Path::new(CONFIG_DATA_FILE).parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    fs::write(CONFIG_DATA_FILE, json).map_err(|e| e.to_string())
}

/// Loads the username from the config file.
///
/// Returns `"User"` as a fallback if the file does not exist.
///
/// # Errors
/// Returns a [`String`] error message if reading or deserializing the file fails.
pub fn load_name() -> Result<String, String> {
    if !Path::new(CONFIG_DATA_FILE).exists() {
        return Ok(DEFAULT_NAME.to_string());
    }

    let json = fs::read_to_string(CONFIG_DATA_FILE).map_err(|e| e.to_string())?;

    if json.trim().is_empty() {
        return Ok(DEFAULT_NAME.to_string());
    }

    let config: Config = serde_json::from_str(&json).map_err(|e| e.to_string())?;
    Ok(config.name)
}