# tellme by Xeij- A terminal cool reads explorer

A Rust terminal application that displays fascinating content from Wikipedia with a typewriter effect. Discover knowledge across **30+ diverse topics** with **5000+ content units** - the ultimate learning companion!

## Features

**30+ Diverse Topics**: Facts, History, Philosophy, Mysteries, Conspiracies, Science, Traditions, Crimes, Civilizations, Psychology, Technology, Medicine, Space, Mathematics, Art, Music, Literature, Film, Architecture, Animals, Biology, Geography, Environment, Weather, Religion, Mythology, Politics, Economics, Sports, Food, Language, Transportation, Inventions, Fashion

**Typewriter Effect**: Content appears character by character with a smooth typing animation

**Smart Recommendations**: Machine learning-like preference tracking that improves content suggestions based on your reading habits

**Persistent Storage**: SQLite database stores content and user interactions locally

**Minimal TUI**: Clean black background with white text and horizontal separators

**Performance Tracking**: Monitors reading time and skip patterns to optimize future content delivery


This is a **knowledge universe** with:

- **30+ Topic Categories**: From Ancient Civilizations to Modern Technology
- **5000+ Content Units**: Massive variety across all topics  
- **25+ Search Queries per Topic**: Specific, curated content for maximum interest
- **Intelligent Content Processing**: Optimized for 1-2 paragraph digestible units
- **Zero Repetition**: Vast content pool means fresh discoveries every session

## Quick Start

### 1. Install Rust

If you don't have Rust installed:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### 2. Clone and Build

```bash
git clone <your-repo-url>
cd tellme
cargo build --release
```

### 3. Fetch Content 

**WARNING**: The 10x expansion will download **5000+ Wikipedia articles**. This will take **30-60 minutes** and require a stable internet connection.

```bash
cargo run --bin fetch_data
```

This massive operation will:
- Create a `tellme_data/` directory
- Download content from Wikipedia's API across 30+ topics
- Process articles into 1-2 paragraph units
- Store everything in a local SQLite database
- Take 30-60 minutes to complete (be patient!)
- Result in ~50-100MB database with 5000+ content units

### 4. Run the App

```bash
cargo run --bin tellme
```

##  **Content Breakdown by Category**

| Category | Topics | Example Content |
|----------|--------|-----------------|
| ** Science & Tech** | Technology, Medicine, Space, Mathematics, Science | AI, Quantum Mechanics, Mars Missions, DNA Discovery |
| ** Arts & Culture** | Art, Music, Literature, Film, Architecture | Renaissance Art, Classical Music, Shakespeare, Hollywood |
| ** Nature & Life** | Animals, Biology, Geography, Environment, Weather | Wildlife, Evolution, Natural Disasters, Climate Change |
| ** Human Society** | Religion, Mythology, Politics, Economics, Sports | World Religions, Greek Myths, Democracy, Olympic Games |
| ** Knowledge & History** | Facts, History, Philosophy, Mysteries, Civilizations | World Wars, Ancient Rome, Unsolved Mysteries, Philosophy |
| ** Daily Life** | Food, Language, Transportation, Inventions, Fashion | World Cuisines, Linguistics, Aviation, Innovation |

## Making `tellme` Globally Executable

### Option 1: Install with Cargo

```bash
cargo install --path .
```

Now you can run `tellme` from anywhere in your terminal!

### Option 2: Manual Installation (Linux/macOS)

```bash
# Build the release binary
cargo build --release

# Copy to a directory in your PATH
sudo cp target/release/tellme /usr/local/bin/

# Make it executable
sudo chmod +x /usr/local/bin/tellme
```

### Option 3: Windows

```powershell
# Build the release binary
cargo build --release

# Copy to a directory in your PATH (adjust path as needed)
copy target\release\tellme.exe C:\Windows\System32\
```

Or add the `target/release` directory to your Windows PATH environment variable.

## How to Use

### Controls

- **Right Arrow (→)**: Skip typewriter animation or proceed to next content
- **Space/Enter**: Same as right arrow
- **Q or Esc**: Quit the application

### Reading Flow

1. Content appears with a typewriter effect
2. Read at your own pace or skip the animation
3. Press right arrow when ready for new content
4. The app learns your preferences across 30+ topics
5. Enjoy discovering new knowledge from 5000+ units!

##  **Advanced Learning Features**

### Smart Content Recommendation
- Tracks reading vs. skipping for each of 30+ topics
- Builds preference profile across diverse subject areas
- Recommends content from your favorite topic clusters
- Adapts to your learning style and interests

### Massive Content Variety
- **150 units per topic** = 4500+ total content units
- **25+ search queries per topic** for maximum diversity
- **Intelligent content filtering** for optimal reading length
- **No repetition** - vast content pool ensures freshness

## Project Structure

```
tellme/
├── Cargo.toml          # Dependencies and project configuration
├── README.md           # This file
├── run_tellme.bat      # Windows helper script
├── run_fetch_data.bat  # Windows data fetcher script
├── src/
│   ├── main.rs         # Main TUI application
│   ├── lib.rs          # Shared library code
│   ├── database.rs     # SQLite operations (handles 5000+ units)
│   ├── content.rs      # 30+ topic definitions and search queries
│   ├── ui.rs           # Terminal UI components
│   └── bin/
│       └── fetch_data.rs   # Wikipedia content fetcher (10x expansion)
└── tellme_data/        # Created by fetch_data
    └── tellme.db       # Massive SQLite database (50-100MB)
```


### 1. Ownership and Borrowing
- **Ownership**: See how `ContentUnit` owns its `String` fields across 5000+ units
- **Borrowing**: Notice `&self` vs `&mut self` in method signatures
- **Move semantics**: Content is moved when setting new content in the app

### 2. Error Handling
- **Result type**: Every fallible operation returns `Result<T, E>`
- **? operator**: Used extensively for error propagation
- **anyhow crate**: Provides convenient error handling

### 3. Pattern Matching
- **match expressions**: Used extensively for 30+ topic handling
- **if let**: For optional value extraction
- **matches! macro**: For checking enum variants

### 4. Async Programming
- **async/await**: The data fetcher uses async HTTP requests for 5000+ articles
- **tokio runtime**: Handles async execution
- **Rate limiting**: Demonstrates async delays across massive content fetch

### 5. Database Operations
- **SQLite integration**: Handles 5000+ content units efficiently
- **Query optimization**: Indexes and efficient retrieval
- **Data persistence**: User preferences across 30+ topics

## Performance Considerations

- **Massive Scale**: Optimized for 5000+ content units
- **Rate Limiting**: 500ms delay between Wikipedia requests (respectful)
- **Batch Processing**: Multiple content units per article
- **Efficient Queries**: Database indexes on topic field for fast retrieval
- **Memory Management**: Lazy loading prevents memory bloat
- **Concurrent Processing**: Async operations for faster data fetching

## **Expected Database Size**

- **Content Units**: 4500+ units across 30+ topics
- **Database Size**: 50-100MB SQLite file
- **Fetch Time**: 30-60 minutes for complete database
- **Articles Processed**: 1000+ unique Wikipedia articles
- **Search Queries**: 750+ different search terms used

## Troubleshooting

### "No content found in database"
Run the data fetcher first: `cargo run --bin fetch_data`
**Note**: This will take 30-60 minutes for the full 10x expansion!

### Network errors during fetch
- Check internet connection
- Wikipedia might be temporarily unavailable
- The fetcher includes automatic retries
- Consider running overnight for the massive fetch

### Large database size
- The 10x expansion creates a 50-100MB database
- This is normal for 5000+ content units
- Ensure you have adequate disk space

### Compilation errors
- Ensure you have the latest Rust version: `rustup update`
- Clear cargo cache: `cargo clean`

## Contributing

Feel free to:

- Add new topics to the 30+ topic enum
- Improve the content processing algorithm for even more variety
- Enhance the UI with more visual elements
- Add keyboard shortcuts
- Implement export features for the massive content database

## Acknowledgments

- Wikipedia for providing free access to human knowledge
- The Rust community for excellent crates and documentation
- Terminal UI inspiration from various TUI applications

---

**Ready to explore 5000+ fascinating knowledge units?** 

Happy learning with Rust and the massively expanded tellme! Start with `cargo run --bin fetch_data`
