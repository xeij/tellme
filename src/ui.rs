// ui.rs - Terminal User Interface components
// This module demonstrates ratatui usage, event handling,
// and asynchronous programming patterns in Rust

use crate::ContentUnit;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame, Terminal,
};
use std::{
    io::{self, Stdout},
    time::{Duration, Instant},
};

/// Main application state
/// This struct demonstrates state management in TUI applications
pub struct App {
    /// Current content being displayed
    pub current_content: Option<ContentUnit>,
    /// Characters displayed so far (for typewriter effect)
    pub displayed_chars: usize,
    /// Whether the current content is fully displayed
    pub fully_displayed: bool,
    /// Start time for measuring reading duration
    pub start_time: Instant,
    /// Whether the app should quit
    pub should_quit: bool,
    /// Status message to display
    pub status_message: String,
}

impl App {
    /// Create a new App instance
    pub fn new() -> Self {
        Self {
            current_content: None,
            displayed_chars: 0,
            fully_displayed: false,
            start_time: Instant::now(),
            should_quit: false,
            status_message: "Loading content...".to_string(),
        }
    }

    /// Set new content to display
    /// This demonstrates method chaining and ownership transfer
    pub fn set_content(&mut self, content: ContentUnit) {
        self.current_content = Some(content);
        self.displayed_chars = 0;
        self.fully_displayed = false;
        self.start_time = Instant::now();
        self.status_message.clear();
    }

    /// Update the typewriter effect
    /// This demonstrates time-based state updates
    pub fn update_typewriter(&mut self) {
        if let Some(ref content) = self.current_content {
            if !self.fully_displayed {
                let total_chars = content.content.len();
                if self.displayed_chars < total_chars {
                    // Display characters gradually (adjust speed here)
                    let chars_per_update = 2; // Characters to reveal per update
                    self.displayed_chars = (self.displayed_chars + chars_per_update).min(total_chars);
                } else {
                    self.fully_displayed = true;
                }
            }
        }
    }

    /// Skip to full content display
    pub fn skip_typewriter(&mut self) {
        if let Some(ref content) = self.current_content {
            self.displayed_chars = content.content.len();
            self.fully_displayed = true;
        }
    }

    /// Get the elapsed reading time in seconds
    pub fn get_reading_time(&self) -> u32 {
        self.start_time.elapsed().as_secs() as u32
    }

    /// Check if content is being displayed
    pub fn has_content(&self) -> bool {
        self.current_content.is_some()
    }

    /// Set status message
    pub fn set_status(&mut self, message: String) {
        self.status_message = message;
    }
}

/// Initialize the terminal for TUI mode
/// This demonstrates terminal setup and error handling
pub fn init_terminal() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend)
}

/// Restore the terminal to normal mode
/// This demonstrates cleanup and the Drop trait concept
pub fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

/// Handle keyboard input events
/// This demonstrates event handling and pattern matching
pub fn handle_events(app: &mut App) -> io::Result<()> {
    // Non-blocking event polling
    if event::poll(Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            // Only handle key press events, not release
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        app.should_quit = true;
                    }
                    KeyCode::Right | KeyCode::Enter | KeyCode::Char(' ') => {
                        if app.has_content() {
                            if !app.fully_displayed {
                                // Skip typewriter effect
                                app.skip_typewriter();
                            } else {
                                // Request new content (handled in main loop)
                                app.current_content = None;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

/// Render the main UI
/// This demonstrates complex layout management and widget composition
pub fn render_ui(frame: &mut Frame, app: &App) {
    let size = frame.size();

    // Create main layout with margins for a clean look
    let main_area = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(1), // Status bar
            Constraint::Length(1), // Separator
            Constraint::Min(0),    // Content area
            Constraint::Length(1), // Help text
        ])
        .split(size);

    // Render status bar
    render_status_bar(frame, app, main_area[0]);

    // Render separator line
    render_separator(frame, main_area[1]);

    // Render main content
    render_content(frame, app, main_area[2]);

    // Render help text
    render_help(frame, app, main_area[3]);
}

/// Render the status bar
fn render_status_bar(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let status_text = if app.status_message.is_empty() {
        if let Some(ref content) = app.current_content {
            format!("Topic: {} | Words: {}", content.topic, content.word_count)
        } else {
            "tellme - Random Knowledge from Wikipedia".to_string()
        }
    } else {
        app.status_message.clone()
    };

    let status = Paragraph::new(status_text)
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center);

    frame.render_widget(status, area);
}

/// Render the separator line
fn render_separator(frame: &mut Frame, area: ratatui::layout::Rect) {
    let separator = Paragraph::new("─".repeat(area.width as usize))
        .style(Style::default().fg(Color::DarkGray));
    
    frame.render_widget(separator, area);
}

/// Render the main content area
fn render_content(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    if let Some(ref content) = app.current_content {
        // Create content layout
        let content_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Title
                Constraint::Min(0),    // Content
            ])
            .split(area);

        // Render title
        let title = Paragraph::new(vec![
            Line::from(Span::styled(
                &content.title,
                Style::default().fg(Color::Cyan),
            )),
        ])
        .block(Block::default().borders(Borders::NONE))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

        frame.render_widget(title, content_layout[0]);

        // Render content with typewriter effect
        let displayed_content = if app.displayed_chars > 0 {
            let chars: Vec<char> = content.content.chars().collect();
            let end_idx = app.displayed_chars.min(chars.len());
            chars[..end_idx].iter().collect::<String>()
        } else {
            String::new()
        };

        // Add cursor if still typing
        let content_text = if !app.fully_displayed && !displayed_content.is_empty() {
            format!("{}▋", displayed_content) // Add block cursor
        } else {
            displayed_content
        };

        let content_paragraph = Paragraph::new(content_text)
            .style(Style::default().fg(Color::White))
            .wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::NONE));

        frame.render_widget(content_paragraph, content_layout[1]);
    } else {
        // Show loading or instructions
        let loading_text = if app.status_message.contains("Loading") {
            vec![
                Line::from(""),
                Line::from(Span::styled(
                    "Loading interesting content...",
                    Style::default().fg(Color::Yellow),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    "Please wait while we fetch knowledge from Wikipedia",
                    Style::default().fg(Color::DarkGray),
                )),
            ]
        } else {
            vec![
                Line::from(""),
                Line::from(Span::styled(
                    "Welcome to tellme!",
                    Style::default().fg(Color::Cyan),
                )),
                Line::from(""),
                Line::from("Discover fascinating facts, mysteries, and knowledge"),
                Line::from("from the depths of Wikipedia."),
                Line::from(""),
                Line::from(Span::styled(
                    "Press any key to start your journey...",
                    Style::default().fg(Color::Green),
                )),
            ]
        };

        let welcome = Paragraph::new(loading_text)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        frame.render_widget(welcome, area);
    }
}

/// Render help text at the bottom
fn render_help(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let help_text = if app.has_content() {
        if app.fully_displayed {
            "→ Next • Space/Enter Next • Q Quit"
        } else {
            "→ Skip typing • Q Quit"
        }
    } else {
        "Any key to start • Q Quit"
    };

    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);

    frame.render_widget(help, area);
}

/// Calculate typing speed for the typewriter effect
/// This demonstrates time-based calculations
pub fn calculate_typing_delay(content_length: usize) -> Duration {
    // Adjust typing speed based on content length
    // Longer content types faster to avoid very long waits
    let base_delay_ms = if content_length > 1000 {
        20 // Fast for long content
    } else if content_length > 500 {
        35 // Medium for medium content
    } else {
        50 // Slower for short content
    };
    
    Duration::from_millis(base_delay_ms)
} 