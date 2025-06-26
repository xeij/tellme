// main.rs - Main application entry point
// This demonstrates the main event loop, error handling,
// and integration of all application components

use anyhow::Result;
use std::time::Duration;
use tellme::{
    database::Database,
    ui::{handle_events, init_terminal, render_ui, restore_terminal, App},
    UserInteraction, DB_FILE,
};

/// Main application entry point
/// This demonstrates Rust's main function and async/await patterns
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize data directory and database
    tellme::ensure_data_dir()?;
    
    // Check if we have any content in the database
    let db = Database::new(DB_FILE)?;
    let content_count = db.get_content_count()?;
    
    if content_count == 0 {
        eprintln!("No content found in database!");
        eprintln!("Please run the data fetcher first:");
        eprintln!("cargo run --bin fetch_data");
        eprintln!();
        eprintln!("This will download and process Wikipedia articles for all topics.");
        return Ok(());
    }

    println!("Found {} content units in database", content_count);
    println!("Starting tellme...");

    // Initialize terminal
    let mut terminal = init_terminal()
        .map_err(|e| anyhow::anyhow!("Failed to initialize terminal: {}", e))?;

    // Create application state
    let mut app = App::new();
    
    // Load initial content
    if let Some(content) = db.get_weighted_random_content()? {
        app.set_content(content);
    } else {
        app.set_status("No content available. Please run fetch_data first.".to_string());
    }

    // Main event loop
    let result = run_app(&mut terminal, &mut app, &db).await;

    // Restore terminal
    restore_terminal(&mut terminal)
        .map_err(|e| anyhow::anyhow!("Failed to restore terminal: {}", e))?;

    // Print final message
    println!("Thanks for using tellme! Keep learning!");

    result
}

/// Main application loop
/// This demonstrates the event loop pattern and state management
async fn run_app(
    terminal: &mut ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>,
    app: &mut App,
    db: &Database,
) -> Result<()> {
    let mut last_update = std::time::Instant::now();
    let update_interval = Duration::from_millis(50); // 20 FPS

    loop {
        // Handle input events
        handle_events(app)?;

        // Check if user wants to quit
        if app.should_quit {
            break;
        }

        // Update typewriter effect
        let now = std::time::Instant::now();
        if now.duration_since(last_update) >= update_interval {
            app.update_typewriter();
            last_update = now;
        }

        // Check if we need new content
        if !app.has_content() && !app.should_quit {
            // Record interaction with previous content if any
            if let Some(ref content) = app.current_content {
                let reading_time = app.get_reading_time();
                let interaction = if app.fully_displayed && reading_time >= 3 {
                    // Consider it "fully read" if they saw it all and spent some time
                    UserInteraction::fully_read(content.id, reading_time)
                } else {
                    // Otherwise, consider it skipped
                    UserInteraction::skipped(content.id, reading_time)
                };
                
                if let Err(e) = db.record_interaction(&interaction) {
                    eprintln!("Warning: Failed to record interaction: {}", e);
                }
            }

            // Load new content
            app.set_status("Loading new content...".to_string());
            
            match db.get_weighted_random_content() {
                Ok(Some(content)) => {
                    app.set_content(content);
                }
                Ok(None) => {
                    app.set_status("No more content available.".to_string());
                }
                Err(e) => {
                    app.set_status(format!("Error loading content: {}", e));
                }
            }
        }

        // Render the UI
        terminal.draw(|frame| render_ui(frame, app))?;

        // Small delay to prevent excessive CPU usage
        tokio::time::sleep(Duration::from_millis(16)).await; // ~60 FPS
    }

    // Record final interaction if there was content being viewed
    if let Some(ref content) = app.current_content {
        let reading_time = app.get_reading_time();
        let interaction = if app.fully_displayed && reading_time >= 3 {
            UserInteraction::fully_read(content.id, reading_time)
        } else {
            UserInteraction::skipped(content.id, reading_time)
        };
        
        if let Err(e) = db.record_interaction(&interaction) {
            eprintln!("Warning: Failed to record final interaction: {}", e);
        }
    }

    Ok(())
} 