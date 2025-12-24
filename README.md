# tellme

A desktop application for browsing historical events and stories from 21 distinct time periods.

## How It Works

The application is built in Rust and operates as a standalone binary with a local SQLite database.

### Core Architecture

The system consists of two main components:
1. **Data Ingestion**: A fetcher utility that queries Wikipedia APIs for historical events. It processes raw text to strip metadata, checks for minimum length requirements, and categorizes content into defined historical eras.
2. **Runtime Application**: The main executable that connects to the local database and renders the interface.

### Content Selection Algorithm

Rather than simple randomization, the application uses a weighted selection system. It tracks user behavior—specifically which stories are finished versus skipped—to adjust the probability of showing similar topics in the future.

### The Stack

- **Language**: Rust
- **GUI Framework**: egui (Immediate Mode GUI for native rendering)
- **Database**: SQLite (via rusqlite)
- **Networking**: reqwest (for initial data fetching)

## Installation

You need the Rust toolchain installed on your machine.

Clone the repository and build the release binary:

```bash
git clone https://github.com/xeij/tellme.git
cd tellme
cargo build --release
```

## Usage

First, populate your local database. This pulls content from the source and builds the SQLite file:

```bash
cargo run --bin fetch_data
```

Once the data is ready, run the application:

```bash
cargo run
```

This launches the GUI. You can navigate stories using the buttons or arrow keys.
