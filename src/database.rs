// database.rs - SQLite database operations
// This module demonstrates Rust's error handling, SQL operations,
// and working with external crates like rusqlite

use crate::{ContentUnit, Topic, UserInteraction, Result};
use rusqlite::{params, Connection, Row, OptionalExtension};
use std::collections::HashMap;

/// Database wrapper that handles all SQLite operations
/// This struct demonstrates Rust's ownership and encapsulation
pub struct Database {
    conn: Connection,
}

impl Database {
    /// Create a new database connection and initialize tables
    /// This demonstrates error propagation with the ? operator
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        let db = Self { conn };
        db.init_tables()?;
        Ok(db)
    }

    /// Initialize database tables if they don't exist
    /// This demonstrates multi-line SQL strings and transaction handling
    fn init_tables(&self) -> Result<()> {
        // Create content table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS content (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                topic TEXT NOT NULL,
                title TEXT NOT NULL,
                content TEXT NOT NULL,
                source_url TEXT NOT NULL,
                word_count INTEGER NOT NULL,
                created_at TEXT NOT NULL
            )",
            [],
        )?;

        // Create user_interactions table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS user_interactions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                content_id INTEGER NOT NULL,
                interaction_type TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                duration_seconds INTEGER NOT NULL,
                FOREIGN KEY (content_id) REFERENCES content (id)
            )",
            [],
        )?;

        // Create index for better query performance
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_content_topic ON content (topic)",
            [],
        )?;

        Ok(())
    }

    /// Insert a new content unit into the database
    /// This demonstrates parameter binding and returning generated IDs
    pub fn insert_content(&self, content: &mut ContentUnit) -> Result<()> {
        let topic_str = serde_json::to_string(&content.topic)?;
        let created_at_str = content.created_at.to_rfc3339();

        let id = self.conn.query_row(
            "INSERT INTO content (topic, title, content, source_url, word_count, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)
             RETURNING id",
            params![
                topic_str,
                content.title,
                content.content,
                content.source_url,
                content.word_count,
                created_at_str
            ],
            |row| row.get::<_, i64>(0),
        )?;

        content.id = id;
        Ok(())
    }

    /// Get a random content unit, weighted by user preferences
    /// This demonstrates complex SQL queries and random selection
    pub fn get_weighted_random_content(&self) -> Result<Option<ContentUnit>> {
        // Get topic preferences based on user interactions
        let topic_weights = self.get_topic_preferences()?;
        
        // If no preferences exist, return truly random content
        if topic_weights.is_empty() {
            return self.get_random_content();
        }

        // Build weighted query - this is a simplified approach
        // In a real app, you might want more sophisticated weighted random selection
        let mut best_topic = Topic::Facts;
        let mut best_score = 0.0;
        
        for (topic, score) in topic_weights {
            if score > best_score {
                best_score = score;
                best_topic = topic;
            }
        }

        self.get_random_content_by_topic(best_topic)
    }

    /// Get completely random content
    fn get_random_content(&self) -> Result<Option<ContentUnit>> {
        self.conn
            .query_row(
                "SELECT id, topic, title, content, source_url, word_count, created_at
                 FROM content
                 ORDER BY RANDOM()
                 LIMIT 1",
                [],
                |row| self.row_to_content_unit(row),
            )
            .optional()
            .map_err(Into::into)
    }

    /// Get random content from a specific topic
    fn get_random_content_by_topic(&self, topic: Topic) -> Result<Option<ContentUnit>> {
        let topic_str = serde_json::to_string(&topic)?;
        
        self.conn
            .query_row(
                "SELECT id, topic, title, content, source_url, word_count, created_at
                 FROM content
                 WHERE topic = ?1
                 ORDER BY RANDOM()
                 LIMIT 1",
                params![topic_str],
                |row| self.row_to_content_unit(row),
            )
            .optional()
            .map_err(Into::into)
    }

    /// Convert a database row to a ContentUnit
    /// This demonstrates error handling within row mapping
    fn row_to_content_unit(&self, row: &Row) -> rusqlite::Result<ContentUnit> {
        let topic_str: String = row.get(1)?;
        let topic: Topic = serde_json::from_str(&topic_str)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(
                1, 
                rusqlite::types::Type::Text, 
                Box::new(e)
            ))?;

        let created_at_str: String = row.get(6)?;
        let created_at = chrono::DateTime::parse_from_rfc3339(&created_at_str)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(
                6, 
                rusqlite::types::Type::Text, 
                Box::new(e)
            ))?
            .with_timezone(&chrono::Utc);

        Ok(ContentUnit {
            id: row.get(0)?,
            topic,
            title: row.get(2)?,
            content: row.get(3)?,
            source_url: row.get(4)?,
            word_count: row.get(5)?,
            created_at,
        })
    }

    /// Record a user interaction with content
    /// This demonstrates enum serialization and database transactions
    pub fn record_interaction(&self, interaction: &UserInteraction) -> Result<()> {
        let (interaction_type, content_id, timestamp, duration) = match interaction {
            UserInteraction::FullyRead { content_id, timestamp, reading_time_seconds } => {
                ("fully_read", *content_id, timestamp, *reading_time_seconds)
            }
            UserInteraction::Skipped { content_id, timestamp, skip_time_seconds } => {
                ("skipped", *content_id, timestamp, *skip_time_seconds)
            }
        };

        self.conn.execute(
            "INSERT INTO user_interactions (content_id, interaction_type, timestamp, duration_seconds)
             VALUES (?1, ?2, ?3, ?4)",
            params![
                content_id,
                interaction_type,
                timestamp.to_rfc3339(),
                duration
            ],
        )?;

        Ok(())
    }

    /// Calculate topic preferences based on user interactions
    /// This demonstrates data aggregation and HashMap usage
    fn get_topic_preferences(&self) -> Result<HashMap<Topic, f64>> {
        let mut stmt = self.conn.prepare(
            "SELECT c.topic, ui.interaction_type, COUNT(*) as count
             FROM user_interactions ui
             JOIN content c ON ui.content_id = c.id
             GROUP BY c.topic, ui.interaction_type"
        )?;

        let rows = stmt.query_map([], |row| {
            let topic_str: String = row.get(0)?;
            let interaction_type: String = row.get(1)?;
            let count: i64 = row.get(2)?;
            Ok((topic_str, interaction_type, count))
        })?;

        let mut topic_stats: HashMap<Topic, (i64, i64)> = HashMap::new(); // (fully_read, skipped)

        for row_result in rows {
            let (topic_str, interaction_type, count) = row_result?;
            let topic: Topic = serde_json::from_str(&topic_str)?;
            
            let entry = topic_stats.entry(topic).or_insert((0, 0));
            match interaction_type.as_str() {
                "fully_read" => entry.0 += count,
                "skipped" => entry.1 += count,
                _ => {} // Ignore unknown interaction types
            }
        }

        // Calculate preference scores (simple ratio of fully_read to total)
        let mut preferences = HashMap::new();
        for (topic, (fully_read, skipped)) in topic_stats {
            let total = fully_read + skipped;
            if total > 0 {
                let score = fully_read as f64 / total as f64;
                preferences.insert(topic, score);
            }
        }

        Ok(preferences)
    }

    /// Get the total number of content units in the database
    pub fn get_content_count(&self) -> Result<i64> {
        let count = self.conn.query_row(
            "SELECT COUNT(*) FROM content",
            [],
            |row| row.get::<_, i64>(0),
        )?;
        Ok(count)
    }

    /// Check if we have content for all topics
    pub fn has_content_for_all_topics(&self) -> Result<bool> {
        let topic_count = self.conn.query_row(
            "SELECT COUNT(DISTINCT topic) FROM content",
            [],
            |row| row.get::<_, i64>(0),
        )?;
        
        Ok(topic_count == Topic::all().len() as i64)
    }
} 