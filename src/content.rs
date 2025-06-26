// content.rs - Data structures and operations for content management
// This module demonstrates Rust's enum system, struct definitions,
// and the derive macro for automatic trait implementations

use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents the different topics we fetch content about
/// This enum demonstrates Rust's powerful enum system - now with 30+ topics!
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Topic {
    // Original topics
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
    
    // NEW: Technology & Innovation
    Technology,
    Medicine,
    Space,
    Mathematics,
    
    // NEW: Arts & Culture
    Art,
    Music,
    Literature,
    Film,
    Architecture,
    
    // NEW: Nature & Life
    Animals,
    Biology,
    Geography,
    Environment,
    Weather,
    
    // NEW: Human Society
    Religion,
    Mythology,
    Politics,
    Economics,
    Sports,
    
    // NEW: Daily Life & Modern World
    Food,
    Language,
    Transportation,
    Inventions,
    Fashion,
}

impl Topic {
    /// Returns all available topics as a slice
    /// This is a const function, computed at compile time - now with 30+ topics!
    pub const fn all() -> &'static [Topic] {
        &[
            // Original topics
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
            
            // Technology & Innovation
            Topic::Technology,
            Topic::Medicine,
            Topic::Space,
            Topic::Mathematics,
            
            // Arts & Culture
            Topic::Art,
            Topic::Music,
            Topic::Literature,
            Topic::Film,
            Topic::Architecture,
            
            // Nature & Life
            Topic::Animals,
            Topic::Biology,
            Topic::Geography,
            Topic::Environment,
            Topic::Weather,
            
            // Human Society
            Topic::Religion,
            Topic::Mythology,
            Topic::Politics,
            Topic::Economics,
            Topic::Sports,
            
            // Daily Life & Modern World
            Topic::Food,
            Topic::Language,
            Topic::Transportation,
            Topic::Inventions,
            Topic::Fashion,
        ]
    }

    /// Get Wikipedia search queries for each topic - MASSIVELY EXPANDED for 10x content!
    /// This demonstrates pattern matching with match expressions
    pub fn search_queries(&self) -> &'static [&'static str] {
        match self {
            Topic::Facts => &[
                "World record", "Guinness World Records", "Strange phenomena", "Unusual animals", 
                "Natural wonders", "Scientific facts", "Human body facts", "Space facts", 
                "Ocean mysteries", "Animal behavior", "Plant adaptations", "Weather phenomena",
                "Rare diseases", "Extinct animals", "Deep sea creatures", "Extreme weather",
                "Human oddities", "Geological wonders", "Astronomical phenomena", "Record holders"
            ],
            Topic::History => &[
                "World War I", "World War II", "Ancient Rome", "Medieval period", "Renaissance",
                "Industrial Revolution", "Cold War", "Viking Age", "Mongol Empire", "Byzantine Empire",
                "Ottoman Empire", "British Empire", "Spanish Empire", "French Revolution", "American Civil War",
                "Russian Revolution", "Napoleon Bonaparte", "Alexander the Great", "Julius Caesar",
                "Cleopatra", "Crusades", "Black Death", "Age of Exploration", "Colonial America",
                "Wild West", "Great Depression", "Holocaust", "Berlin Wall", "Pearl Harbor"
            ],
            Topic::Philosophy => &[
                "Socrates", "Plato", "Aristotle", "Descartes", "Kant", "Nietzsche", "Existentialism",
                "Stoicism", "Buddhism philosophy", "Confucianism", "Ethics", "Metaphysics", "Epistemology",
                "Logic", "Phenomenology", "Utilitarianism", "Nihilism", "Determinism", "Free will",
                "Moral philosophy", "Political philosophy", "Philosophy of mind", "Ancient philosophy"
            ],
            Topic::Mysteries => &[
                "Bermuda Triangle", "Stonehenge", "Easter Island", "Nazca Lines", "Crop circles",
                "Ball lightning", "Spontaneous human combustion", "Voynich Manuscript", "Antikythera mechanism",
                "Shroud of Turin", "Oak Island", "Bigfoot", "UFO sightings", "Ghost phenomena",
                "Loch Ness Monster", "Atlantis", "Tunguska event", "Mary Celeste", "Lost colonies",
                "Time travel", "Poltergeist", "Near-death experiences", "Psychic phenomena"
            ],
            Topic::Conspiracies => &[
                "JFK assassination", "Moon landing conspiracy", "9/11 conspiracy", "Area 51", "Illuminati",
                "New World Order", "Chemtrails", "HAARP", "MKUltra", "Project Blue Book", "Roswell incident",
                "Philadelphia Experiment", "Flat Earth", "QAnon", "Reptilian people", "Bohemian Grove",
                "Denver Airport", "FEMA camps", "Subliminal messages", "Mind control"
            ],
            Topic::Science => &[
                "DNA discovery", "Theory of relativity", "Quantum mechanics", "Evolution", "Penicillin",
                "Vaccines", "Atomic theory", "Periodic table", "Electromagnetic radiation", "Black holes",
                "Big Bang theory", "Photosynthesis", "Genetics", "Stem cells", "CRISPR", "Antibiotics",
                "Nuclear fusion", "Particle physics", "Climate change", "Radioactivity", "X-rays",
                "Laser technology", "Superconductivity", "Gene therapy", "Nanotechnology"
            ],
            Topic::Traditions => &[
                "Japanese tea ceremony", "Diwali", "Day of the Dead", "Carnival", "Chinese New Year",
                "Oktoberfest", "Holi festival", "Thanksgiving", "Aboriginal Dreamtime", "Native American traditions",
                "Celtic festivals", "African tribal customs", "Hindu traditions", "Buddhist ceremonies",
                "Christmas traditions", "Wedding customs", "Coming of age rituals", "Funeral rites",
                "Harvest festivals", "Spring festivals", "Religious pilgrimages", "Folk dances"
            ],
            Topic::Crimes => &[
                "Jack the Ripper", "Zodiac Killer", "Black Dahlia", "D.B. Cooper", "Lindbergh kidnapping",
                "Alcatraz escape", "Great Train Robbery", "Art theft", "Ponzi scheme", "Watergate scandal",
                "Enron scandal", "Al Capone", "Pablo Escobar", "Serial killers", "Bonnie and Clyde",
                "Bank robberies", "Heists", "Cold cases", "Forensic science", "Criminal psychology",
                "White collar crime", "Organized crime", "Cybercrime", "Identity theft"
            ],
            Topic::Civilizations => &[
                "Ancient Egypt", "Maya civilization", "Aztec Empire", "Inca Empire", "Mesopotamia",
                "Indus Valley Civilization", "Ancient Greece", "Roman Empire", "Persian Empire", "Chinese dynasties",
                "Viking civilization", "Angkor Wat", "Machu Picchu", "Petra", "Pompeii", "Troy",
                "Babylonian Empire", "Assyrian Empire", "Phoenicians", "Celts", "Germanic tribes",
                "Silk Road", "Great Wall of China", "Pyramids", "Colosseum", "Parthenon"
            ],
            Topic::Psychology => &[
                "Cognitive bias", "Memory formation", "Dreams", "Consciousness", "Personality psychology",
                "Social psychology", "Behavioral psychology", "Pavlov experiments", "Stanford prison experiment",
                "Milgram experiment", "Placebo effect", "Optical illusions", "Phobias", "Depression", "Autism",
                "PTSD", "Schizophrenia", "Multiple personality", "Learning disabilities", "Intelligence",
                "Emotion", "Stress", "Sleep disorders", "Addiction psychology", "Child psychology"
            ],
            
            // NEW TOPIC QUERIES - MASSIVE EXPANSION!
            Topic::Technology => &[
                "Internet", "World Wide Web", "Computer", "Smartphone", "Artificial Intelligence",
                "Machine learning", "Robotics", "Virtual reality", "Augmented reality", "3D printing",
                "Blockchain", "Cryptocurrency", "Social media", "Cloud computing", "Internet of Things",
                "Quantum computing", "Cybersecurity", "Big data", "Programming languages", "Operating systems",
                "Video games", "Software engineering", "Hardware", "Semiconductors", "Fiber optics"
            ],
            Topic::Medicine => &[
                "Heart surgery", "Cancer treatment", "Organ transplant", "Medical imaging", "Vaccines",
                "Antibiotics", "Anesthesia", "Surgery", "Mental health", "Genetics medicine",
                "Immunology", "Pharmacology", "Epidemiology", "Public health", "Medical devices",
                "Telemedicine", "Precision medicine", "Medical ethics", "Clinical trials", "Disease prevention",
                "Emergency medicine", "Pediatrics", "Geriatrics", "Medical breakthroughs", "Health technology"
            ],
            Topic::Space => &[
                "NASA", "Space exploration", "Apollo missions", "International Space Station", "Mars missions",
                "Hubble telescope", "Solar system", "Planets", "Stars", "Galaxies", "Nebulae",
                "Astronauts", "Space shuttle", "Satellites", "Rocket technology", "Space race",
                "Exoplanets", "Space colonization", "Asteroid belt", "Comets", "Space debris",
                "Dark matter", "Dark energy", "Cosmic radiation", "Space travel", "SETI"
            ],
            Topic::Mathematics => &[
                "Calculus", "Algebra", "Geometry", "Statistics", "Probability", "Number theory",
                "Mathematical proofs", "Famous mathematicians", "Mathematical constants", "Infinity",
                "Fractals", "Game theory", "Cryptography", "Mathematical modeling", "Topology",
                "Set theory", "Logic", "Differential equations", "Linear algebra", "Mathematical paradoxes",
                "Prime numbers", "Mathematical discoveries", "Applications of mathematics", "Pure mathematics"
            ],
            Topic::Art => &[
                "Leonardo da Vinci", "Michelangelo", "Pablo Picasso", "Vincent van Gogh", "Claude Monet",
                "Renaissance art", "Impressionism", "Abstract art", "Modern art", "Contemporary art",
                "Sculpture", "Painting techniques", "Art movements", "Famous paintings", "Art museums",
                "Street art", "Digital art", "Photography", "Art history", "Art criticism",
                "Installation art", "Performance art", "Art collecting", "Art restoration", "Folk art"
            ],
            Topic::Music => &[
                "Classical music", "Mozart", "Beethoven", "Bach", "Jazz music", "Blues", "Rock music",
                "Pop music", "Hip hop", "Electronic music", "Folk music", "Country music", "Opera",
                "Symphony orchestra", "Musical instruments", "Music theory", "Composers", "Musicians",
                "Music genres", "Music history", "Music festivals", "Record industry", "Music technology",
                "Sound engineering", "Music therapy", "World music", "Musical notation"
            ],
            Topic::Literature => &[
                "Shakespeare", "Mark Twain", "Jane Austen", "Charles Dickens", "Ernest Hemingway",
                "Poetry", "Novels", "Short stories", "Drama", "Fiction", "Non-fiction", "Biographies",
                "Literary criticism", "Book publishing", "Libraries", "Reading", "Writing", "Authors",
                "Literary movements", "Classic literature", "Modern literature", "Children's literature",
                "Science fiction", "Fantasy literature", "Mystery novels", "Romance novels"
            ],
            Topic::Film => &[
                "Hollywood", "Cinema history", "Film directors", "Movie stars", "Academy Awards",
                "Film genres", "Special effects", "Animation", "Documentary films", "Independent films",
                "Foreign films", "Film festivals", "Movie studios", "Cinematography", "Film editing",
                "Sound design", "Film production", "Movie theaters", "Film criticism", "Cult films",
                "Silent films", "Color films", "Digital cinema", "Film preservation", "Movie industry"
            ],
            Topic::Architecture => &[
                "Ancient architecture", "Gothic architecture", "Renaissance architecture", "Modern architecture",
                "Skyscrapers", "Famous buildings", "Architectural styles", "Urban planning", "Construction",
                "Building materials", "Architectural engineering", "Interior design", "Landscape architecture",
                "Sustainable architecture", "Architectural history", "Monuments", "Bridges", "Dams",
                "Cathedrals", "Palaces", "Castles", "Architectural preservation", "Building codes"
            ],
            Topic::Animals => &[
                "Mammals", "Birds", "Reptiles", "Amphibians", "Fish", "Insects", "Marine life",
                "Endangered species", "Animal behavior", "Animal intelligence", "Animal communication",
                "Migration patterns", "Predators", "Ecosystems", "Zoos", "Wildlife conservation",
                "Pets", "Domestic animals", "Wild animals", "Animal rights", "Veterinary medicine",
                "Animal training", "Animal psychology", "Extinct animals", "Animal evolution"
            ],
            Topic::Biology => &[
                "Cell biology", "Molecular biology", "Genetics", "Evolution", "Ecology", "Botany",
                "Microbiology", "Biochemistry", "Physiology", "Anatomy", "Neuroscience", "Immunology",
                "Marine biology", "Biotechnology", "Biodiversity", "Photosynthesis", "DNA", "RNA",
                "Proteins", "Enzymes", "Hormones", "Metabolism", "Reproduction", "Development biology"
            ],
            Topic::Geography => &[
                "Continents", "Countries", "Mountains", "Rivers", "Oceans", "Deserts", "Forests",
                "Islands", "Volcanoes", "Earthquakes", "Climate", "Weather patterns", "Natural disasters",
                "Cartography", "GPS", "Exploration", "Geographic features", "Topography", "Geology",
                "Human geography", "Urban geography", "Population", "Demographics", "Migration", "Borders"
            ],
            Topic::Environment => &[
                "Climate change", "Global warming", "Pollution", "Renewable energy", "Conservation",
                "Sustainability", "Recycling", "Deforestation", "Ocean pollution", "Air quality",
                "Water conservation", "Environmental policy", "Green technology", "Carbon footprint",
                "Ecosystem", "Biodiversity loss", "Environmental activism", "Natural resources",
                "Environmental science", "Ecology", "Environmental protection", "Waste management"
            ],
            Topic::Weather => &[
                "Hurricanes", "Tornadoes", "Thunderstorms", "Blizzards", "Droughts", "Floods",
                "Meteorology", "Weather forecasting", "Climate patterns", "Atmospheric science",
                "Extreme weather", "Weather instruments", "Clouds", "Precipitation", "Temperature",
                "Wind patterns", "Seasonal changes", "Weather phenomena", "Storm chasing", "Weather safety"
            ],
            Topic::Religion => &[
                "Christianity", "Islam", "Judaism", "Buddhism", "Hinduism", "Religious history",
                "Religious practices", "Religious texts", "Theology", "Comparative religion",
                "Religious philosophy", "Spirituality", "Meditation", "Prayer", "Religious festivals",
                "Religious art", "Religious music", "Monasticism", "Pilgrimage", "Religious leaders",
                "Religious conflicts", "Religious tolerance", "Secularism", "Atheism", "Agnosticism"
            ],
            Topic::Mythology => &[
                "Greek mythology", "Roman mythology", "Norse mythology", "Egyptian mythology", "Celtic mythology",
                "Hindu mythology", "Chinese mythology", "Japanese mythology", "Native American mythology",
                "African mythology", "Mesopotamian mythology", "Mythical creatures", "Creation myths",
                "Hero myths", "Flood myths", "Underworld myths", "Sky gods", "Earth goddesses",
                "Trickster gods", "Mythological symbols", "Folklore", "Legends", "Fairy tales"
            ],
            Topic::Politics => &[
                "Democracy", "Political systems", "Government", "Elections", "Political parties",
                "Political leaders", "International relations", "Diplomacy", "Political theory",
                "Political history", "Constitutional law", "Civil rights", "Human rights", "Political movements",
                "Revolution", "Political economy", "Public policy", "Political science", "Voting systems",
                "Political campaigns", "Political corruption", "Political reform", "Governance"
            ],
            Topic::Economics => &[
                "Capitalism", "Socialism", "Economic systems", "Stock market", "Banking", "Currency",
                "Inflation", "Recession", "Economic theory", "Microeconomics", "Macroeconomics",
                "International trade", "Economic development", "Economic policy", "Economic history",
                "Labor economics", "Economic inequality", "Economic growth", "Market economics",
                "Economic indicators", "Financial markets", "Economic cycles", "Entrepreneurship"
            ],
            Topic::Sports => &[
                "Olympic Games", "Football", "Basketball", "Baseball", "Soccer", "Tennis", "Golf",
                "Swimming", "Track and field", "Boxing", "Wrestling", "Martial arts", "Gymnastics",
                "Figure skating", "Skiing", "Cycling", "Racing", "Extreme sports", "Team sports",
                "Individual sports", "Sports history", "Sports psychology", "Sports medicine",
                "Athletic training", "Sports equipment", "Sports rules", "Professional sports"
            ],
            Topic::Food => &[
                "Cooking techniques", "World cuisines", "Food history", "Nutrition", "Restaurants",
                "Food science", "Culinary arts", "Food safety", "Food production", "Agriculture",
                "Organic food", "Fast food", "Food culture", "Traditional dishes", "Spices",
                "Beverages", "Food preservation", "Food allergies", "Dietary requirements", "Food trends",
                "Food industry", "Celebrity chefs", "Food photography", "Food criticism", "Food festivals"
            ],
            Topic::Language => &[
                "Linguistics", "Language families", "Grammar", "Vocabulary", "Phonetics", "Semantics",
                "Language evolution", "Dead languages", "Endangered languages", "Translation", "Interpretation",
                "Bilingualism", "Language learning", "Writing systems", "Alphabet", "Typography",
                "Communication", "Sign language", "Body language", "Language disorders", "Sociolinguistics",
                "Psycholinguistics", "Computational linguistics", "Language policy", "Dialects"
            ],
            Topic::Transportation => &[
                "Automobiles", "Aircraft", "Ships", "Trains", "Bicycles", "Public transportation",
                "Transportation history", "Aviation", "Maritime transport", "Railway systems", "Roads",
                "Traffic", "Transportation engineering", "Logistics", "Supply chain", "Transportation policy",
                "Electric vehicles", "Autonomous vehicles", "Transportation safety", "Urban transportation",
                "Freight transport", "Passenger transport", "Transportation infrastructure", "Future transport"
            ],
            Topic::Inventions => &[
                "Steam engine", "Light bulb", "Telephone", "Radio", "Television", "Computer", "Internet",
                "Printing press", "Wheel", "Compass", "Gunpowder", "Paper", "Microscope", "Telescope",
                "Photography", "Motion pictures", "Refrigeration", "Air conditioning", "Medical inventions",
                "Military inventions", "Agricultural inventions", "Communication inventions", "Transportation inventions",
                "Energy inventions", "Modern inventions", "Future inventions", "Patent system", "Innovation"
            ],
            Topic::Fashion => &[
                "Fashion history", "Fashion designers", "Fashion trends", "Clothing", "Textiles",
                "Fashion industry", "Fashion shows", "Fashion photography", "Fashion journalism",
                "Sustainable fashion", "Fast fashion", "Luxury fashion", "Street fashion", "Fashion accessories",
                "Fashion technology", "Fashion business", "Fashion marketing", "Fashion education",
                "Cultural fashion", "Fashion psychology", "Fashion and identity", "Fashion movements"
            ],
        }
    }
}

/// Display implementation for Topic - demonstrates trait implementation
impl fmt::Display for Topic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            // Original topics
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
            
            // Technology & Innovation
            Topic::Technology => "Technology",
            Topic::Medicine => "Medicine & Health",
            Topic::Space => "Space & Astronomy",
            Topic::Mathematics => "Mathematics",
            
            // Arts & Culture
            Topic::Art => "Art & Visual Arts",
            Topic::Music => "Music",
            Topic::Literature => "Literature",
            Topic::Film => "Film & Cinema",
            Topic::Architecture => "Architecture",
            
            // Nature & Life
            Topic::Animals => "Animals & Wildlife",
            Topic::Biology => "Biology & Life Sciences",
            Topic::Geography => "Geography",
            Topic::Environment => "Environment & Ecology",
            Topic::Weather => "Weather & Climate",
            
            // Human Society
            Topic::Religion => "Religion & Spirituality",
            Topic::Mythology => "Mythology & Folklore",
            Topic::Politics => "Politics & Government",
            Topic::Economics => "Economics & Finance",
            Topic::Sports => "Sports & Athletics",
            
            // Daily Life & Modern World
            Topic::Food => "Food & Cuisine",
            Topic::Language => "Language & Linguistics",
            Topic::Transportation => "Transportation",
            Topic::Inventions => "Inventions & Innovation",
            Topic::Fashion => "Fashion & Style",
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