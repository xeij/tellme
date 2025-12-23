// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Mutex;

// Import from parent crate - we'll need to copy the necessary modules
mod database;
mod content;

use content::{ContentUnit, Topic, UserInteraction};
use database::Database;

/// Application state
struct AppState {
    db: Mutex<Database>,
}

/// Response for content statistics
#[derive(Serialize)]
struct StatsResponse {
    total_content: i64,
    total_interactions: i64,
}

/// Request for recording interactions
#[derive(Deserialize)]
struct InteractionRequest {
    content_id: i64,
    fully_read: bool,
    reading_time_seconds: u32,
}

/// Tauri command: Get random weighted content
#[tauri::command]
fn get_random_content(state: tauri::State<AppState>) -> Result<ContentUnit, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    
    match db.get_weighted_random_content() {
        Ok(Some(content)) => Ok(content),
        Ok(None) => Err("No content available".to_string()),
        Err(e) => Err(format!("Database error: {}", e)),
    }
}

/// Tauri command: Record user interaction
#[tauri::command]
fn record_interaction(
    state: tauri::State<AppState>,
    content_id: i64,
    fully_read: bool,
    reading_time_seconds: u32,
) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    
    let interaction = if fully_read {
        UserInteraction::fully_read(content_id, reading_time_seconds)
    } else {
        UserInteraction::skipped(content_id, reading_time_seconds)
    };
    
    db.record_interaction(&interaction)
        .map_err(|e| format!("Failed to record interaction: {}", e))
}

/// Tauri command: Get content statistics
#[tauri::command]
fn get_stats(state: tauri::State<AppState>) -> Result<StatsResponse, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    
    let total_content = db
        .get_content_count()
        .map_err(|e| format!("Failed to get content count: {}", e))?;
    
    Ok(StatsResponse {
        total_content,
        total_interactions: 0,
    })
}

fn main() {
    // Get database path
    let db_path = get_db_path();
    
    // Ensure data directory exists
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent).expect("Failed to create data directory");
    }
    
    // Initialize database
    let db = Database::new(&db_path.to_string_lossy())
        .expect("Failed to initialize database");
    
    // Check if we have content
    let content_count = db.get_content_count().unwrap_or(0);
    if content_count == 0 {
        eprintln!("Warning: No content found in database!");
        eprintln!("Please run: cargo run --bin fetch_data");
    }
    
    tauri::Builder::default()
        .manage(AppState {
            db: Mutex::new(db),
        })
        .invoke_handler(tauri::generate_handler![
            get_random_content,
            record_interaction,
            get_stats
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Get the database file path
fn get_db_path() -> PathBuf {
    let mut path = std::env::current_exe()
        .unwrap_or_else(|_| PathBuf::from("."));
    path.pop(); // Remove executable name
    path.push("tellme_data");
    path.push("tellme.db");
    path
}
