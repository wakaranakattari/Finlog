use crate::utils::*;
use std::fmt;
use std::io;

/// Application-level error type that formats messages using [`color_error_print`].
#[derive(Debug)]
pub struct AppError(String);

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", color_error_print(&self.0))
    }
}

impl From<io::Error> for AppError {
    fn from(e: io::Error) -> Self {
        AppError(e.to_string())
    }
}

impl From<String> for AppError {
    fn from(e: String) -> Self {
        AppError(e)
    }
}