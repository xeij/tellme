// tellme_web.rs - Web server for tellme application
// Provides a web-based interface for viewing historical content

use anyhow::Result;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tellme::{database::Database, ContentUnit, UserInteraction, DB_FILE};
use tower_http::services::ServeDir;

/// Application state shared across handlers
#[derive(Clone)]
struct AppState {
    db: Arc<Database>,
}

/// Response for content statistics
#[derive(Serialize)]
struct StatsResponse {
    total_content: i64,
    total_interactions: i64,
}

/// Request body for recording interactions
#[derive(Deserialize)]
struct InteractionRequest {
    content_id: i64,
    fully_read: bool,
    reading_time_seconds: u32,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("️ tellme Web Server");
    println!("======================");

    // Initialize data directory and database
    tellme::ensure_data_dir()?;
    let db = Database::new(DB_FILE)?;
    
    // Check if we have content
    let content_count = db.get_content_count()?;
    if content_count == 0 {
        eprintln!(" No content found in database!");
        eprintln!("Please run the data fetcher first:");
        eprintln!("cargo run --bin fetch_data");
        return Ok(());
    }

    println!(" Found {} content units in database", content_count);

    // Create shared application state
    let state = AppState {
        db: Arc::new(db),
    };

    // Build router with API routes and static file serving
    let app = Router::new()
        .route("/api/content/random", get(get_random_content))
        .route("/api/content/:id/interaction", post(record_interaction))
        .route("/api/stats", get(get_stats))
        .nest_service("/", ServeDir::new("static"))
        .with_state(state);

    // Start server
    let addr = "127.0.0.1:3000";
    println!(" Server running at http://{}", addr);
    println!(" Open your browser to start reading history!");
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Handler: Get random weighted content
async fn get_random_content(
    State(state): State<AppState>,
) -> Result<Json<ContentUnit>, (StatusCode, String)> {
    match state.db.get_weighted_random_content() {
        Ok(Some(content)) => Ok(Json(content)),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            "No content available".to_string(),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )),
    }
}

/// Handler: Record user interaction with content
async fn record_interaction(
    State(state): State<AppState>,
    Json(req): Json<InteractionRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let interaction = if req.fully_read {
        UserInteraction::fully_read(req.content_id, req.reading_time_seconds)
    } else {
        UserInteraction::skipped(req.content_id, req.reading_time_seconds)
    };

    match state.db.record_interaction(&interaction) {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to record interaction: {}", e),
        )),
    }
}

/// Handler: Get content statistics
async fn get_stats(
    State(state): State<AppState>,
) -> Result<Json<StatsResponse>, (StatusCode, String)> {
    let total_content = state
        .db
        .get_content_count()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // For now, we'll return 0 for interactions (can be enhanced later)
    let stats = StatsResponse {
        total_content,
        total_interactions: 0,
    };

    Ok(Json(stats))
}
