# tellme - Fascinating History from All Ages ️

A terminal application that displays engaging historical content from Prehistoric times to the Contemporary era.

## Features

- 🏛️ **21 Historical Periods**: From Prehistoric times through Ancient civilizations to modern Contemporary history
- 🌐 **Dual Interface**: Choose between terminal or modern web interface
- 🔄 **Auto-Update System**: Automatically checks GitHub for new releases on startup
- ⭐ **Quality Content Filtering**: Sophisticated scoring system that prioritizes fascinating, engaging historical stories over boring encyclopedia entries
- ⌨️ **Typewriter Effect**: Beautiful animated text display that makes reading more engaging
- 🎯 **Smart Recommendations**: AI-powered system that learns your preferences and recommends content you'll find interesting
- 💾 **SQLite Database**: Efficiently stores and manages historical content with user interaction tracking
- 🔍 **Intelligent Content Processing**: Filters Wikipedia articles to extract the most compelling historical facts, stories, and discoveries

## Historical Periods Covered

### Prehistoric & Ancient Times
- **Prehistoric** (Before written history) - Amazing discoveries, cave paintings, ancient human achievements
- **Ancient Egypt** (3100-30 BCE) - Pharaoh secrets, pyramid mysteries, incredible discoveries
- **Ancient Greece** (800-146 BCE) - Philosopher secrets, Olympic origins, incredible inventions
- **Ancient Rome** (753 BCE-476/1453 CE) - Emperor secrets, gladiator stories, engineering marvels
- **Ancient China** (2070 BCE-220 CE) - Dynasty secrets, Great Wall mysteries, incredible inventions

### Classical & Post-Classical Era
- **Byzantine** (330-1453 CE) - Emperor secrets, Constantinople mysteries, incredible treasures
- **Medieval** (500-1500 CE) - King secrets, knight stories, incredible inventions
- **Viking** (793-1066 CE) - Warrior secrets, exploration adventures, incredible discoveries
- **Islamic** (610-1258 CE) - Empire secrets, scientific discoveries, architectural wonders
- **Mongol** (1206-1368 CE) - Empire secrets, Genghis Khan stories, incredible conquests

### Early Modern Period
- **Renaissance** (1300-1600 CE) - Artist secrets, incredible inventions, amazing discoveries
- **Age of Exploration** (1400-1600 CE) - Explorer secrets, navigation innovations, incredible adventures
- **Colonial** (1492-1800 CE) - Colonial secrets, amazing discoveries, incredible adventures
- **Enlightenment** (1685-1815 CE) - Philosopher secrets, revolutionary ideas, incredible breakthroughs

### Modern Era
- **Industrial** (1760-1840 CE) - Revolutionary secrets, amazing inventions, incredible innovations
- **19th Century** (1801-1900 CE) - Century secrets, amazing inventions, incredible discoveries
- **World War I** (1914-1918 CE) - War secrets, heroic stories, incredible technology
- **Interwar Period** (1918-1939 CE) - Era secrets, 1920s discoveries, incredible culture
- **World War II** (1939-1945 CE) - War secrets, heroic stories, incredible operations
- **Cold War** (1947-1991 CE) - Espionage secrets, amazing technology, incredible mysteries
- **Contemporary** (1991-present) - Modern secrets, recent discoveries, incredible innovations

## Content Quality System

The app uses a sophisticated content scoring system that prioritizes:

### High-Quality Content (Prioritized)
- **Discoveries & Mysteries**: Hidden secrets, shocking revelations, amazing discoveries
- **Drama & Intrigue**: Betrayals, conspiracies, scandals, assassinations, rebellions
- **Human Interest**: Heroic stories, survival tales, adventures, legends, romances
- **Unusual & Bizarre**: Strange events, unique achievements, rare occurrences
- **Innovation & Achievement**: Brilliant inventions, revolutionary breakthroughs, triumphs
- **Superlatives**: Record-breaking events, greatest achievements, most famous figures

###  Low-Quality Content (Filtered Out)
- Boring encyclopedia entries and dry academic text
- List articles and disambiguation pages
- Technical documentation without engaging stories
- Reference sections and citation-heavy content

## Installation

### Prerequisites
- Rust (install from [rustup.rs](https://rustup.rs))

### Quick Install (Recommended)
```bash
# Clone the repository
git clone https://github.com/xeij/tellme.git
cd tellme

# Install globally (adds to your PATH)
cargo install --path .

# Now you can run from anywhere
tellme
```

### Manual Build
```bash
# Clone and build
git clone https://github.com/xeij/tellme.git
cd tellme
cargo build --release

# Run directly
./target/release/tellme
```

## Usage

### First Time Setup
1. **Install the app** (see installation above)
2. **Fetch historical content**:
   ```bash
   fetch_data
   ```
   This downloads ~525 fascinating historical content units (25 per historical period)
   
3. **Run the app**:
   ```bash
   tellme
   ```

### Web Interface (NEW! 🌐)

For a modern web-based experience with premium UI:

1. **Start the web server**:
   ```bash
   cargo run --bin tellme-web
   # Or if installed globally:
   tellme-web
   ```

2. **Open your browser**:
   Navigate to `http://localhost:3000`

3. **Enjoy the experience**:
   - Beautiful dark theme with glassmorphism effects
   - Smooth typewriter animations
   - Keyboard shortcuts (Space/Enter for next, Esc to skip)
   - Progress tracking and statistics

### Terminal vs Web Interface

| Feature | Terminal | Web |
|---------|----------|-----|
| Typewriter Effect | ✅ | ✅ |
| Dark Theme | ✅ | ✅ Premium |
| Keyboard Shortcuts | ✅ | ✅ |
| Visual Animations | Basic | Advanced |
| Accessibility | High | High |
| Resource Usage | Minimal | Light |


### Controls
- **Space/Enter**: Skip to next content
- **Escape/Q**: Quit application
- **Auto-advance**: Content automatically advances when fully displayed

### Content Database
- **Size**: ~525 high-quality historical content units
- **Storage**: ~15-25MB SQLite database
- **Quality**: Advanced filtering ensures only engaging, fascinating content
- **Coverage**: Balanced across all 21 historical periods

## How It Works

### Smart Content Selection
The app uses machine learning-style recommendation system:
1. **Quality Filtering**: Only processes high-scoring content with engaging keywords
2. **User Learning**: Tracks what you read vs skip to learn your preferences  
3. **Balanced Discovery**: Ensures variety while respecting your interests
4. **Historical Balance**: Provides content from all time periods

### Content Processing Pipeline
1. **Wikipedia Search**: Targets fascinating historical topics with engaging search terms
2. **Quality Scoring**: Each article gets scored for engagement, drama, and interest level
3. **Content Extraction**: Extracts the most compelling paragraphs and stories
4. **Length Optimization**: Ensures perfect reading length (30-800 words)
5. **Database Storage**: Saves processed content with metadata and user interaction tracking

### Auto-Update System
- Checks GitHub releases on startup (5-second timeout)
- Notifies you of available updates with download instructions
- Non-intrusive - continues if update check fails

## Project Structure

```
tellme/
├── src/
│   ├── main.rs           # Main TUI application with auto-update
│   ├── lib.rs            # Library definitions and exports
│   ├── content.rs        # Historical periods and content structures
│   ├── database.rs       # SQLite operations and smart recommendations
│   ├── ui.rs             # Terminal UI and typewriter effects
│   ├── auto_update.rs    # GitHub release checking
│   └── bin/
│       ├── fetch_data.rs # Wikipedia content fetcher with quality filtering
│       └── tellme_web.rs # Web server for browser-based interface
├── static/               # Web UI files
│   ├── index.html        # Main web interface
│   ├── styles.css        # Premium dark theme styling
│   └── app.js            # Frontend logic and typewriter effect
├── tellme_data/          # Database storage directory
└── Cargo.toml           # Dependencies and project configuration
```

---

*"Those who cannot remember the past are condemned to repeat it." - George Santayana*

