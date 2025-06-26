# tellme - Terminal Knowledge Explorer

A Rust terminal application that displays fascinating content from Wikipedia with a typewriter effect. Learn about interesting facts, history, philosophy, mysteries, and more.

## Features

🎯 **10 Diverse Topics**: Facts, History, Philosophy, Mysteries, Conspiracies, Science, Cultural Traditions, Unsolved Crimes, Ancient Civilizations, and Psychology

⌨️ **Typewriter Effect**: Content appears character by character with a smooth typing animation

🧠 **Smart Recommendations**: Machine learning-like preference tracking that improves content suggestions based on your reading habits

💾 **Persistent Storage**: SQLite database stores content and user interactions locally

🎨 **Minimal TUI**: Clean black background with white text and elegant horizontal separators

📊 **Performance Tracking**: Monitors reading time and skip patterns to optimize future content delivery

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

First, download and process Wikipedia articles:
```bash
cargo run --bin fetch_data
```

This will:
- Create a `tellme_data/` directory
- Download content from Wikipedia's API
- Process articles into 1-2 paragraph units
- Store everything in a local SQLite database
- Take several minutes to complete (be patient!)

### 4. Run the App

```bash
cargo run --bin tellme
```

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
4. The app learns your preferences over time
5. Enjoy discovering new knowledge!

## Project Structure

```
tellme/
├── Cargo.toml          # Dependencies and project configuration
├── README.md           # This file
├── src/
│   ├── main.rs         # Main TUI application
│   ├── lib.rs          # Shared library code
│   ├── database.rs     # SQLite operations
│   ├── content.rs      # Content structures and topic definitions
│   ├── ui.rs           # Terminal UI components
│   └── bin/
│       └── fetch_data.rs   # Wikipedia content fetcher
└── tellme_data/        # Created by fetch_data
    └── tellme.db       # SQLite database
```

## How the Recommendation System Works

The app tracks two types of interactions:

1. **Fully Read**: Content displayed completely for 3+ seconds
2. **Skipped**: Content skipped quickly or not fully displayed

The system calculates a preference score for each topic:
```
preference_score = fully_read_count / total_interactions
```

Topics with higher scores are more likely to be selected in future sessions.

## Content Processing Pipeline

1. **Search**: Query Wikipedia's OpenSearch API for article titles
2. **Fetch**: Get article content using the MediaWiki API
3. **Parse**: Extract plain text and split into paragraphs
4. **Filter**: Keep only suitable 1-2 paragraph units (50-500 words)
5. **Clean**: Remove citations and normalize formatting
6. **Store**: Save to SQLite with metadata

## Performance Considerations

- **Rate Limiting**: 500ms delay between Wikipedia requests
- **Batch Processing**: Multiple content units per article
- **Efficient Queries**: Database indexes on topic field
- **Lazy Loading**: Content fetched only when needed

## Troubleshooting

### "No content found in database"
Run the data fetcher first: `cargo run --bin fetch_data`

### Network errors during fetch
- Check internet connection
- Wikipedia might be temporarily unavailable
- The fetcher includes automatic retries

### Compilation errors
- Ensure you have the latest Rust version: `rustup update`
- Clear cargo cache: `cargo clean`

### Terminal display issues
- Ensure your terminal supports Unicode
- Try a different terminal emulator
- Check that your terminal is large enough

## Contributing

P.S feel free to:

- Add new topics to the `Topic` enum
- Improve the content processing algorithm
- Enhance the UI with more visual elements
- Add keyboard shortcuts
- Implement export features

## Acknowledgments

- Wikipedia for providing free access to human knowledge
- The Rust community for excellent crates and documentation
- Terminal UI inspiration from various TUI applications

---

Happy learning with tellme!
