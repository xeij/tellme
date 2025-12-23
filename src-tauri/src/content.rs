// content.rs - Data structures and operations for content management
// This module demonstrates Rust's enum system, struct definitions,
// and the derive macro for automatic trait implementations

use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents different historical time periods and eras we focus on
/// This enum demonstrates Rust's powerful enum system - focused on HISTORY!
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Topic {
    // Prehistoric & Ancient Times
    Prehistoric,        // Before written history
    AncientEgypt,      // 3100 BCE - 30 BCE
    AncientGreece,     // 800 BCE - 146 BCE
    AncientRome,       // 753 BCE - 476/1453 CE
    AncientChina,      // 2070 BCE - 220 CE
    
    // Classical & Post-Classical
    Byzantine,         // 330-1453 CE
    Medieval,          // 500-1500 CE
    Viking,           // 793-1066 CE
    Islamic,          // 610-1258 CE
    Mongol,           // 1206-1368 CE
    
    // Early Modern Period
    Renaissance,       // 1300-1600 CE
    AgeOfExploration, // 1400-1600 CE
    Colonial,         // 1492-1800 CE
    Enlightenment,    // 1685-1815 CE
    
    // Modern Era
    Industrial,       // 1760-1840 CE
    NineteenthCentury, // 1801-1900 CE
    WorldWarOne,      // 1914-1918 CE
    InterwarPeriod,   // 1918-1939 CE
    WorldWarTwo,      // 1939-1945 CE
    ColdWar,          // 1947-1991 CE
    Contemporary,     // 1991-present
}

impl Topic {
    /// Returns all historical periods as a slice
    pub const fn all() -> &'static [Topic] {
        &[
            // Prehistoric & Ancient Times
            Topic::Prehistoric,
            Topic::AncientEgypt,
            Topic::AncientGreece,
            Topic::AncientRome,
            Topic::AncientChina,
            
            // Classical & Post-Classical
            Topic::Byzantine,
            Topic::Medieval,
            Topic::Viking,
            Topic::Islamic,
            Topic::Mongol,
            
            // Early Modern Period
            Topic::Renaissance,
            Topic::AgeOfExploration,
            Topic::Colonial,
            Topic::Enlightenment,
            
            // Modern Era
            Topic::Industrial,
            Topic::NineteenthCentury,
            Topic::WorldWarOne,
            Topic::InterwarPeriod,
            Topic::WorldWarTwo,
            Topic::ColdWar,
            Topic::Contemporary,
        ]
    }

    /// Get fascinating historical search queries for each time period
    /// Focused on amazing stories, shocking events, incredible people, and mind-blowing discoveries
    pub fn search_queries(&self) -> &'static [&'static str] {
        match self {
            Topic::Prehistoric => &[
                "Prehistoric archaeology", "Stone Age", "Ice Age", "Cave paintings", "Neanderthal",
                "Hunter gatherer", "Megalith", "Stonehenge", "Paleolithic", "Neolithic",
                "Early humans", "Fossil humans", "Ancient tools", "Prehistoric art", "Mammoth"
            ],
            
            Topic::AncientEgypt => &[
                "Ancient Egypt", "Pharaoh", "Pyramid", "Mummy", "Hieroglyph",
                "Tutankhamun", "Cleopatra", "Nile River", "Sphinx", "Egyptian mythology",
                "Egyptian medicine", "Papyrus", "Egyptian gods", "Valley of the Kings", "Egyptian art"
            ],
            
            Topic::AncientGreece => &[
                "Ancient Greece", "Alexander the Great", "Greek philosophy", "Olympic Games", "Sparta",
                "Athens", "Greek mythology", "Parthenon", "Socrates", "Plato",
                "Aristotle", "Greek democracy", "Greek theater", "Greek warfare", "Greek art"
            ],
            
            Topic::AncientRome => &[
                "Roman Empire", "Julius Caesar", "Augustus", "Gladiator", "Colosseum",
                "Roman legion", "Pompeii", "Roman engineering", "Roman law", "Constantine",
                "Fall of Rome", "Roman Senate", "Roman gods", "Roman architecture", "Hadrian's Wall"
            ],
            
            Topic::AncientChina => &[
                "Ancient China", "Great Wall of China", "Chinese dynasty", "Confucius", "Chinese emperor",
                "Silk Road", "Chinese philosophy", "Chinese invention", "Terracotta Army", "Chinese medicine",
                "Chinese art", "Chinese writing", "Chinese warfare", "Forbidden City", "Chinese mythology"
            ],
            
            Topic::Byzantine => &[
                "Byzantine Empire", "Constantinople", "Byzantine emperor", "Hagia Sophia", "Justinian",
                "Byzantine art", "Eastern Orthodox", "Byzantine military", "Byzantine culture", "Crusades",
                "Ottoman conquest", "Byzantine architecture", "Byzantine politics", "Byzantine trade", "Greek fire"
            ],
            
            Topic::Medieval => &[
                "Middle Ages", "Medieval Europe", "Knight", "Castle", "Feudalism",
                "Crusades", "Black Death", "Medieval warfare", "Medieval art", "Gothic architecture",
                "Medieval literature", "Viking raids", "Medieval trade", "Medieval technology", "Medieval church"
            ],
            
            Topic::Viking => &[
                "Viking", "Norse mythology", "Viking exploration", "Viking ship", "Viking raid",
                "Viking settlement", "Norse saga", "Viking culture", "Viking warfare", "Leif Erikson",
                "Viking Age", "Norse gods", "Runes", "Viking trade", "Viking society"
            ],
            
            Topic::Islamic => &[
                "Islamic civilization", "Islamic Golden Age", "Islamic conquest", "Caliphate", "Islamic science",
                "Islamic art", "Islamic architecture", "Islamic philosophy", "Muhammad", "Quran",
                "Islamic empire", "Islamic trade", "Islamic medicine", "Islamic mathematics", "Mosque"
            ],
            
            Topic::Mongol => &[
                "Mongol Empire", "Genghis Khan", "Mongol conquest", "Mongol warfare", "Silk Road",
                "Kublai Khan", "Mongol culture", "Mongol society", "Mongol military", "Yuan dynasty",
                "Mongol invasion", "Mongol administration", "Mongol trade", "Mongol religion", "Pax Mongolica"
            ],
            
            Topic::Renaissance => &[
                "Renaissance", "Leonardo da Vinci", "Michelangelo", "Renaissance art", "Humanism",
                "Italian Renaissance", "Renaissance science", "Printing press", "Renaissance literature", "Medici family",
                "Renaissance architecture", "Renaissance philosophy", "Renaissance technology", "Renaissance exploration", "Renaissance music"
            ],
            
            Topic::AgeOfExploration => &[
                "Age of Exploration", "Christopher Columbus", "Vasco da Gama", "Magellan", "Spanish conquest",
                "Portuguese exploration", "New World", "European exploration", "Maritime exploration", "Colonial empire",
                "Navigation", "Conquistador", "Trading post", "Exploration technology", "Cartography"
            ],
            
            Topic::Colonial => &[
                "Colonial America", "British Empire", "Spanish Empire", "French colonial empire", "Dutch Empire",
                "Colonization", "Colonial society", "Colonial economy", "Colonial culture", "Colonial trade",
                "Colonial administration", "Colonial resistance", "Colonial expansion", "Colonial settlement", "Mercantilism"
            ],
            
            Topic::Enlightenment => &[
                "Age of Enlightenment", "Enlightenment philosophy", "Voltaire", "John Locke", "Scientific Revolution",
                "Enlightenment thinkers", "Political philosophy", "Natural rights", "Social contract", "Reason",
                "Enlightenment science", "Encyclopedia", "Enlightenment politics", "Religious tolerance", "Progress"
            ],
            
            Topic::Industrial => &[
                "Industrial Revolution", "Steam engine", "Factory system", "Industrial technology", "Railway",
                "Industrial society", "Industrial workers", "Textile industry", "Coal mining", "Iron industry",
                "Industrial cities", "Labor movement", "Industrial capitalism", "Mass production", "Industrial innovation"
            ],
            
            Topic::NineteenthCentury => &[
                "19th century", "Victorian era", "Nationalism", "Romanticism", "Scientific progress",
                "Social reform", "Abolition", "Women's rights", "Labor rights", "Political revolution",
                "Cultural change", "Technological advancement", "Economic growth", "Imperial expansion", "Social movement"
            ],
            
            Topic::WorldWarOne => &[
                "World War I", "Trench warfare", "Western Front", "Russian Revolution", "Treaty of Versailles",
                "World War 1 technology", "Military strategy", "War propaganda", "Home front", "War casualties",
                "Assassination of Archduke", "Central Powers", "Allied Powers", "Battle of the Somme", "Armistice"
            ],
            
            Topic::InterwarPeriod => &[
                "Interwar period", "Great Depression", "Rise of fascism", "Weimar Republic", "Soviet Union",
                "Jazz Age", "Roaring Twenties", "Stock market crash", "New Deal", "Appeasement",
                "League of Nations", "Cultural change", "Political instability", "Economic crisis", "Social change"
            ],
            
            Topic::WorldWarTwo => &[
                "World War II", "Holocaust", "D-Day", "Pearl Harbor", "Battle of Britain",
                "Nazi Germany", "Pacific War", "Resistance movement", "War crimes", "Atomic bomb",
                "Blitzkrieg", "Eastern Front", "Home front", "War technology", "Liberation"
            ],
            
            Topic::ColdWar => &[
                "Cold War", "Iron Curtain", "Berlin Wall", "Cuban Missile Crisis", "Space Race",
                "McCarthyism", "Nuclear arms race", "Proxy war", "Decolonization", "Détente",
                "Soviet Union", "NATO", "Warsaw Pact", "Korean War", "Vietnam War"
            ],
            
            Topic::Contemporary => &[
                "Contemporary history", "Globalization", "Digital revolution", "Fall of communism", "Terrorism",
                "Climate change", "Internet", "Social media", "Economic integration", "Cultural diversity",
                "Technological advancement", "Political change", "Social transformation", "Environmental issues", "Human rights"
            ],
        }
    }
}

/// Display implementation for Topic - demonstrates trait implementation
impl fmt::Display for Topic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            // Prehistoric & Ancient Times
            Topic::Prehistoric => "Prehistoric",
            Topic::AncientEgypt => "Ancient Egypt",
            Topic::AncientGreece => "Ancient Greece",
            Topic::AncientRome => "Ancient Rome",
            Topic::AncientChina => "Ancient China",
            
            // Classical & Post-Classical
            Topic::Byzantine => "Byzantine",
            Topic::Medieval => "Medieval",
            Topic::Viking => "Viking",
            Topic::Islamic => "Islamic",
            Topic::Mongol => "Mongol",
            
            // Early Modern Period
            Topic::Renaissance => "Renaissance",
            Topic::AgeOfExploration => "Age of Exploration",
            Topic::Colonial => "Colonial",
            Topic::Enlightenment => "Enlightenment",
            
            // Modern Era
            Topic::Industrial => "Industrial",
            Topic::NineteenthCentury => "19th Century",
            Topic::WorldWarOne => "World War I",
            Topic::InterwarPeriod => "Interwar Period",
            Topic::WorldWarTwo => "World War II",
            Topic::ColdWar => "Cold War",
            Topic::Contemporary => "Contemporary",
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
        let word_count = self.word_count;
        
        // More flexible: suitable if it's 30-800 words (adjusted for better content variety)
        // This allows for both concise and more detailed content
        word_count >= 30 && word_count <= 800
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