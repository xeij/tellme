// lib.rs - Shared library code for tellme application
// This module contains common structures and functionality used by both
// the main TUI app and the data fetching script

pub mod database;
pub mod content;
pub mod ui;

// Re-export commonly used types for convenience
pub use content::{ContentUnit, Topic, UserInteraction};
pub use database::Database;

// Error type alias for easier error handling throughout the app
pub type Result<T> = anyhow::Result<T>;

// Constants used throughout the application
pub const DATA_DIR: &str = "tellme_data";
pub const DB_FILE: &str = "tellme_data/tellme.db";

/// Initialize the data directory if it doesn't exist
/// This demonstrates Rust's Result type and error handling
pub fn ensure_data_dir() -> Result<()> {
    std::fs::create_dir_all(DATA_DIR)?;
    Ok(())
} 