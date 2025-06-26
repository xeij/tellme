// content.rs - Data structures and operations for content management
// This module demonstrates Rust's enum system, struct definitions,
// and the derive macro for automatic trait implementations

use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents the different topics we fetch content about
/// This enum demonstrates Rust's powerful enum system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Topic {
    Facts,
    History,
    Philosophy,
    Mysteries,
    Conspiracies,
    Science,
    Traditions,
    Crimes,
    Civilizations,
    Psychology,
}

impl Topic {
    /// Returns all available topics as a slice
    /// This is a const function, computed at compile time
    pub const fn all() -> &'static [Topic] {
        &[
            Topic::Facts,
            Topic::History,
            Topic::Philosophy,
            Topic::Mysteries,
            Topic::Conspiracies,
            Topic::Science,
            Topic::Traditions,
            Topic::Crimes,
            Topic::Civilizations,
            Topic::Psychology,
        ]
    }

    /// Get Wikipedia search queries for each topic
    /// This demonstrates pattern matching with match expressions
    pub fn search_queries(&self) -> &'static [&'static str] {
        match self {
            Topic::Facts => &["Amazing facts", "Interesting trivia", "Did you know", "Fun facts"],
            Topic::History => &["Historical events", "Ancient history", "World history", "Historic mysteries"],
            Topic::Philosophy => &["Philosophy", "Philosophical concepts", "Ancient philosophy", "Modern philosophy"],
            Topic::Mysteries => &["Unsolved mysteries", "World mysteries", "Mysterious phenomena", "Unexplained events"],
            Topic::Conspiracies => &["Conspiracy theories", "Alternative theories", "Controversial theories"],
            Topic::Science => &["Scientific discoveries", "Science breakthroughs", "Modern science", "Scientific phenomena"],
            Topic::Traditions => &["Cultural traditions", "World cultures", "Ancient traditions", "Folk traditions"],
            Topic::Crimes => &["Unsolved crimes", "Famous crimes", "Criminal cases", "Mystery crimes"],
            Topic::Civilizations => &["Ancient civilizations", "Lost civilizations", "Ancient cultures", "Archaeological discoveries"],
            Topic::Psychology => &["Psychology", "Human behavior", "Psychological phenomena", "Cognitive science"],
        }
    }
}

/// Display implementation for Topic - demonstrates trait implementation
impl fmt::Display for Topic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Topic::Facts => "Interesting Facts",
            Topic::History => "History",
            Topic::Philosophy => "Philosophy",
            Topic::Mysteries => "World Mysteries",
            Topic::Conspiracies => "Conspiracies",
            Topic::Science => "Science",
            Topic::Traditions => "Cultural Traditions",
            Topic::Crimes => "Unsolved Crimes",
            Topic::Civilizations => "Ancient Civilizations",
            Topic::Psychology => "Psychology",
        };
        write!(f, "{}", name)
    }
}

/// Represents a unit of content to display to the user
/// This struct demonstrates Rust's ownership system and the use of String vs &str
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentUnit {
    pub id: i64,
    pub topic: Topic,
    pub title: String,
    pub content: String,
    pub source_url: String,
    pub word_count: usize,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl ContentUnit {
    /// Create a new content unit
    /// This demonstrates the builder pattern and taking ownership of strings
    pub fn new(
        topic: Topic,
        title: String,
        content: String,
        source_url: String,
    ) -> Self {
        let word_count = content.split_whitespace().count();
        
        Self {
            id: 0, // Will be set by database
            topic,
            title,
            content,
            source_url,
            word_count,
            created_at: chrono::Utc::now(),
        }
    }

    /// Check if this content unit is suitable (1-2 paragraphs)
    /// This demonstrates method implementation and borrowing (&self)
    pub fn is_suitable_length(&self) -> bool {
        let paragraph_count = self.content.split("\n\n").count();
        let word_count = self.word_count;
        
        // Suitable if it's 1-2 paragraphs and 50-500 words
        paragraph_count >= 1 && paragraph_count <= 2 && word_count >= 50 && word_count <= 500
    }

    /// Clean the content by removing unwanted characters and formatting
    /// This demonstrates mutable borrowing (&mut self) and string manipulation
    pub fn clean_content(&mut self) {
        // Remove citations like [1], [2], etc.
        let re = regex::Regex::new(r"\[\d+\]").unwrap();
        self.content = re.replace_all(&self.content, "").to_string();
        
        // Remove extra whitespace and normalize line breaks
        self.content = self.content
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("\n\n");
    }
}

/// Represents user interaction with content
/// This demonstrates Rust's enum with data and timestamp handling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserInteraction {
    FullyRead {
        content_id: i64,
        timestamp: chrono::DateTime<chrono::Utc>,
        reading_time_seconds: u32,
    },
    Skipped {
        content_id: i64,
        timestamp: chrono::DateTime<chrono::Utc>,
        skip_time_seconds: u32,
    },
}

impl UserInteraction {
    /// Create a new "fully read" interaction
    pub fn fully_read(content_id: i64, reading_time_seconds: u32) -> Self {
        Self::FullyRead {
            content_id,
            timestamp: chrono::Utc::now(),
            reading_time_seconds,
        }
    }

    /// Create a new "skipped" interaction
    pub fn skipped(content_id: i64, skip_time_seconds: u32) -> Self {
        Self::Skipped {
            content_id,
            timestamp: chrono::Utc::now(),
            skip_time_seconds,
        }
    }

    /// Get the content ID this interaction refers to
    /// This demonstrates pattern matching with references
    pub fn content_id(&self) -> i64 {
        match self {
            Self::FullyRead { content_id, .. } => *content_id,
            Self::Skipped { content_id, .. } => *content_id,
        }
    }

    /// Check if this was a positive interaction (fully read)
    pub fn is_positive(&self) -> bool {
        matches!(self, Self::FullyRead { .. })
    }
} 