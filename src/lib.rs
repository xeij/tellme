// lib.rs - Shared library code for tellme application
// This module contains common structures and functionality used by both
// the main TUI app and the data fetching script

pub mod database;
pub mod content;
pub mod ui;
pub mod auto_update;

// Re-export commonly used types for convenience
pub use content::{ContentUnit, Topic, UserInteraction};
pub use database::Database;

// Error type alias for easier error handling throughout the app
pub type Result<T> = anyhow::Result<T>;

// Constants used throughout the application
pub const DATA_DIR: &str = "tellme_data";
pub const DB_FILE: &str = "tellme_data/tellme.db";

/// Create the data directory if it doesn't exist
/// This demonstrates file system operations and error handling
pub fn ensure_data_dir() -> anyhow::Result<()> {
    let data_dir = std::path::Path::new(DATA_DIR);
    if !data_dir.exists() {
        std::fs::create_dir_all(data_dir)?;
    }
    Ok(())
} 