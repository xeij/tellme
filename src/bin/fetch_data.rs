// fetch_data.rs - Wikipedia content fetcher
// This binary demonstrates HTTP requests, HTML parsing, async programming,
// and data processing in Rust

use anyhow::Result;
use rand::seq::SliceRandom;
use reqwest::Client;
use serde_json::Value;
use std::time::Duration;
use tellme::{
    content::{ContentUnit, Topic},
    database::Database,
    ensure_data_dir, DB_FILE,
};

/// Wikipedia API client for fetching articles
/// This struct demonstrates HTTP client usage and rate limiting
struct WikipediaClient {
    client: Client,
    base_url: String,
}

impl WikipediaClient {
    /// Create a new Wikipedia client
    fn new() -> Self {
        let client = Client::builder()
            .user_agent("tellme/0.1.0 (https://github.com/example/tellme)")
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url: "https://en.wikipedia.org/w/api.php".to_string(),
        }
    }

    /// Search for articles on a given topic
    /// This demonstrates async HTTP requests and JSON parsing
    async fn search_articles(&self, query: &str, limit: usize) -> Result<Vec<String>> {
        let url = format!(
            "{}?action=opensearch&search={}&limit={}&namespace=0&format=json",
            self.base_url,
            urlencoding::encode(query),
            limit
        );

        println!("Searching for: {} (limit: {})", query, limit);

        let response = self.client.get(&url).send().await?;
        let text = response.text().await?;
        
        // Parse the OpenSearch JSON response
        let json: Value = serde_json::from_str(&text)?;
        
        if let Some(titles) = json.get(1).and_then(|v| v.as_array()) {
            let article_titles: Vec<String> = titles
                .iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect();
            
            Ok(article_titles)
        } else {
            Ok(Vec::new())
        }
    }

    /// Get the content of a Wikipedia article
    /// This demonstrates error handling and HTML parsing
    async fn get_article_content(&self, title: &str) -> Result<Option<(String, String)>> {
        let url = format!(
            "{}?action=query&format=json&titles={}&prop=extracts&exintro=&explaintext=&exsectionformat=plain",
            self.base_url,
            urlencoding::encode(title)
        );

        println!("Fetching article: {}", title);

        let response = self.client.get(&url).send().await?;
        let text = response.text().await?;
        
        let json: Value = serde_json::from_str(&text)?;
        
        // Navigate the complex Wikipedia API response structure
        if let Some(pages) = json.get("query").and_then(|q| q.get("pages")) {
            if let Some(page) = pages.as_object().and_then(|obj| obj.values().next()) {
                if let Some(extract) = page.get("extract").and_then(|e| e.as_str()) {
                    let page_url = format!("https://en.wikipedia.org/wiki/{}", 
                                         urlencoding::encode(title));
                    return Ok(Some((extract.to_string(), page_url)));
                }
            }
        }
        
        Ok(None)
    }

    /// Add a small delay between requests to be respectful to Wikipedia
    async fn rate_limit(&self) {
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
}

/// Process article content into suitable units
/// This demonstrates text processing and content validation with QUALITY SCORING
fn process_article_content(
    topic: Topic,
    title: &str,
    content: &str,
    source_url: &str,
) -> Vec<ContentUnit> {
    let mut units = Vec::new();
    
    // First, check content quality score
    let quality_score = calculate_content_quality_score(content, title);
    
    // Only process decent quality, engaging content (score > 0, lowered from 3)
    if quality_score < 0 {
        return units; // Skip truly boring content
    }
    
    // First, try to use the full content if it's not too long
    if content.len() > 100 && content.len() < 3000 {
        let mut full_unit = ContentUnit::new(
            topic,
            title.to_string(),
            content.to_string(),
            source_url.to_string(),
        );
        
        full_unit.clean_content();
        
        if full_unit.is_suitable_length() {
            units.push(full_unit);
            return units; // Return the full content if it's suitable
        }
    }
    
    // If full content is too long, split into sections
    let sections: Vec<&str> = content
        .split("\n\n")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty() && s.len() > 30)
        .collect();

    // Try to create content units from sections
    let mut i = 0;
    while i < sections.len() {
        let mut unit_content = sections[i].to_string();
        
        // If current section is short, try to combine with next sections
        let mut j = i + 1;
        while j < sections.len() && unit_content.len() < 400 {
            unit_content.push_str("\n\n");
            unit_content.push_str(sections[j]);
            j += 1;
        }
        
        // Check quality of this specific unit content
        let unit_quality = calculate_content_quality_score(&unit_content, title);
        if unit_quality < -1 {
            i = if j > i + 1 { j } else { i + 1 };
            continue; // Skip very low-quality sections
        }
        
        let mut content_unit = ContentUnit::new(
            topic,
            title.to_string(),
            unit_content,
            source_url.to_string(),
        );
        
        content_unit.clean_content();
        
        if content_unit.is_suitable_length() {
            units.push(content_unit);
        }
        
        // Move to the next unprocessed section
        i = if j > i + 1 { j } else { i + 1 };
    }
    
    units
}

/// Calculate content quality score based on engaging keywords and patterns
/// Higher scores = more interesting, engaging content
fn calculate_content_quality_score(content: &str, title: &str) -> i32 {
    let content_lower = content.to_lowercase();
    let title_lower = title.to_lowercase();
    let combined = format!("{} {}", title_lower, content_lower);
    
    let mut score = 0;
    
    // BASE SCORE for any historical content (be more generous)
    if content.len() > 50 {
        score += 1; // Base point for having actual content
    }
    
    // POSITIVE INDICATORS - Fascinating, engaging content
    let fascinating_words = [
        // Discovery & Mystery
        "discovered", "mystery", "secret", "hidden", "revealed", "uncovered", "found",
        "breakthrough", "revelation", "shocking", "amazing", "incredible", "extraordinary",
        
        // Drama & Intrigue  
        "betrayal", "conspiracy", "scandal", "plot", "intrigue", "assassination", "murder",
        "rebellion", "revolution", "war", "battle", "siege", "conquest", "victory", "defeat",
        
        // Human Interest
        "heroic", "courage", "brave", "survival", "escape", "rescue", "adventure",
        "legend", "myth", "story", "tale", "epic", "dramatic", "tragic", "romance",
        
        // Unusual & Bizarre
        "strange", "bizarre", "unusual", "weird", "odd", "peculiar", "unique", "rare",
        "first", "last", "only", "never", "always", "forbidden", "lost", "ancient",
        
        // Innovation & Achievement
        "invented", "created", "built", "achieved", "accomplished", "succeeded", "triumph",
        "genius", "brilliant", "innovative", "revolutionary", "groundbreaking",
        
        // Superlatives & Records
        "largest", "smallest", "fastest", "strongest", "richest", "most", "greatest",
        "best", "worst", "famous", "notorious", "legendary", "record", "unprecedented"
    ];
    
    for word in &fascinating_words {
        if combined.contains(word) {
            score += 1;
        }
    }
    
    // BONUS for multiple engaging elements
    if combined.contains("emperor") || combined.contains("king") || combined.contains("queen") {
        score += 1;
    }
    if combined.contains("treasure") || combined.contains("gold") || combined.contains("wealth") {
        score += 1;
    }
    if combined.contains("died") || combined.contains("killed") || combined.contains("death") {
        score += 1;
    }
    if combined.contains("empire") || combined.contains("kingdom") || combined.contains("civilization") {
        score += 1;
    }
    
    // BONUS for historical periods and dates
    if content.contains("BCE") || content.contains("CE") || content.contains("century") || content.contains("AD") {
        score += 2; // Historical content gets bonus points
    }
    
    // BONUS for people and places (historical names)
    if combined.contains("dynasty") || combined.contains("pharaoh") || combined.contains("caesar") {
        score += 1;
    }
    
    // NEGATIVE INDICATORS - Boring, dry content (less harsh)
    let boring_indicators = [
        "list of", "disambiguation", "stub", "citation needed",
        "clarification needed", "template", "infobox", "navbox"
    ];
    
    for indicator in &boring_indicators {
        if combined.contains(indicator) {
            score -= 3; // Still penalize but less harshly
        }
    }
    
    // MILD penalty for overly technical language
    let technical_words = ["according to", "it is believed", "scholars suggest"];
    for word in &technical_words {
        if combined.contains(word) {
            score -= 1;
        }
    }
    
    score
}

/// Fetch content for a specific topic
/// This demonstrates error handling and progress reporting
async fn fetch_topic_content(
    client: &WikipediaClient,
    db: &Database,
    topic: Topic,
    target_count: usize,
) -> Result<usize> {
    println!("\n=== Fetching content for {} ===", topic);
    
    let mut total_units = 0;
    let queries = topic.search_queries();
    
    for query in queries {
        if total_units >= target_count {
            break;
        }
        
        // Search for articles (massive limit increase for 10x content variety)
        let article_titles = client.search_articles(query, 50).await?;
        
        for title in article_titles {
            if total_units >= target_count {
                break;
            }
            
            // Skip disambiguation and list pages
            if title.contains("disambiguation") || title.contains("List of") {
                continue;
            }
            
            client.rate_limit().await;
            
            match client.get_article_content(&title).await {
                Ok(Some((content, url))) => {
                    let units = process_article_content(topic, &title, &content, &url);
                    
                    for mut unit in units {
                        match db.insert_content(&mut unit) {
                            Ok(()) => {
                                total_units += 1;
                                println!("  ✓ Added unit {} from '{}'", total_units, title);
                            }
                            Err(e) => {
                                eprintln!("  ✗ Failed to save unit from '{}': {}", title, e);
                            }
                        }
                        
                        if total_units >= target_count {
                            break;
                        }
                    }
                }
                Ok(None) => {
                    println!("  - No content found for '{}'", title);
                }
                Err(e) => {
                    eprintln!("  ✗ Error fetching '{}': {}", title, e);
                }
            }
        }
    }
    
    println!("Fetched {} units for {}", total_units, topic);
    Ok(total_units)
}

/// Main entry point for the data fetcher
/// This demonstrates the main async function pattern and comprehensive error handling
#[tokio::main]
async fn main() -> Result<()> {
    println!("tellme Data Fetcher");
    println!("==================");
    println!("This will download and process Wikipedia articles for all topics.");
    println!("This may take several minutes...\n");

    // Ensure data directory exists
    ensure_data_dir()?;
    
    // Initialize database
    let db = Database::new(DB_FILE)?;
    
    // Check existing content
    let existing_count = db.get_content_count()?;
    println!("Current database contains {} content units", existing_count);
    
    if existing_count > 0 {
        println!("Database already contains content. This will add more content to it.");
        println!("Continue? (y/N)");
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        
        if !input.trim().to_lowercase().starts_with('y') {
            println!("Cancelled.");
            return Ok(());
        }
    }
    
    // Create Wikipedia client
    let client = WikipediaClient::new();
    
    // Target number of units per topic (REDUCED for focused historical content!)
    // With 21 historical periods, this will give us ~525 total units (quality over quantity)
    let units_per_topic = 25; // 21 topics × 25 units = ~525 total units
    let mut total_fetched = 0;
    
    // Fetch content for each topic
    let topics = Topic::all();
    let mut rng = rand::thread_rng();
    let mut shuffled_topics = topics.to_vec();
    shuffled_topics.shuffle(&mut rng);
    
    for &topic in &shuffled_topics {
        match fetch_topic_content(&client, &db, topic, units_per_topic).await {
            Ok(count) => {
                total_fetched += count;
            }
            Err(e) => {
                eprintln!("Error fetching content for {}: {}", topic, e);
            }
        }
        
        // Brief pause between topics
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
    
    println!("\n=== Summary ===");
    println!("Total content units fetched: {}", total_fetched);
    
    let final_count = db.get_content_count()?;
    println!("Total content units in database: {}", final_count);
    
    if db.has_content_for_all_topics()? {
        println!("✓ All topics have content!");
    } else {
        println!("⚠ Some topics may have limited content");
    }
    
    println!("\nData fetching complete! You can now run:");
    println!("cargo run --bin tellme");
    
    Ok(())
}
