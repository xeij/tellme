// tellme_gui.rs - Minimal desktop GUI using egui
// Simple black background, white text, basic buttons

use eframe::egui;
use tellme::{database::Database, ContentUnit, UserInteraction, DB_FILE};

fn main() -> Result<(), eframe::Error> {
    // Initialize database
    tellme::ensure_data_dir().expect("Failed to create data directory");
    let db = Database::new(DB_FILE).expect("Failed to open database");
    
    let content_count = db.get_content_count().unwrap_or(0);
    if content_count == 0 {
        eprintln!("No content in database. Run: cargo run --bin fetch_data");
    }

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("tellme - History"),
        ..Default::default()
    };

    eframe::run_native(
        "tellme",
        options,
        Box::new(|_cc| Box::new(TellMeApp::new(db))),
    )
}

struct TellMeApp {
    db: Database,
    current_content: Option<ContentUnit>,
    start_time: std::time::Instant,
}

impl TellMeApp {
    fn new(db: Database) -> Self {
        let mut app = Self {
            db,
            current_content: None,
            start_time: std::time::Instant::now(),
        };
        app.load_next_content();
        app
    }

    fn load_next_content(&mut self) {
        // Record interaction with previous content
        if let Some(ref content) = self.current_content {
            let reading_time = self.start_time.elapsed().as_secs() as u32;
            let interaction = UserInteraction::fully_read(content.id, reading_time);
            let _ = self.db.record_interaction(&interaction);
        }

        // Load new content
        match self.db.get_weighted_random_content() {
            Ok(Some(content)) => {
                self.current_content = Some(content);
                self.start_time = std::time::Instant::now();
            }
            _ => {}
        }
    }
}

impl eframe::App for TellMeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Set dark theme
        ctx.set_visuals(egui::Visuals {
            dark_mode: true,
            override_text_color: Some(egui::Color32::WHITE),
            ..egui::Visuals::dark()
        });

        // Handle keyboard input
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowRight) || i.key_pressed(egui::Key::ArrowDown)) {
            self.load_next_content();
        }
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(egui::Color32::BLACK))
            .show(ctx, |ui| {
                // Main scrollable content area
                let bottom_height = 60.0;
                egui::ScrollArea::vertical()
                    .max_height(ui.available_height() - bottom_height)
                    .show(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(20.0);
                            
                            // Title
                            ui.heading(egui::RichText::new("tellme - History").color(egui::Color32::WHITE).size(24.0));
                            
                            ui.add_space(20.0);
                            ui.separator();
                            ui.add_space(20.0);

                            if let Some(ref content) = self.current_content {
                                // Topic badge
                                ui.label(egui::RichText::new(format!("📚 {}", content.topic)).color(egui::Color32::LIGHT_GRAY));
                                
                                ui.add_space(10.0);
                                
                                // Content title
                                ui.label(egui::RichText::new(&content.title).color(egui::Color32::WHITE).size(18.0).strong());
                                
                                ui.add_space(15.0);
                                
                                // Content text
                                ui.label(egui::RichText::new(&content.content).color(egui::Color32::WHITE).size(14.0));
                                
                                ui.add_space(40.0);
                            } else {
                                ui.label(egui::RichText::new("No content available").color(egui::Color32::WHITE));
                                ui.label(egui::RichText::new("Run: cargo run --bin fetch_data").color(egui::Color32::LIGHT_GRAY));
                            }
                        });
                    });

                // Fixed bottom-right buttons
                ui.with_layout(egui::Layout::bottom_up(egui::Align::RIGHT), |ui| {
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        if ui.button(egui::RichText::new("Quit").size(16.0)).clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                        
                        ui.add_space(10.0);
                        
                        if ui.button(egui::RichText::new("Next →").size(16.0)).clicked() {
                            self.load_next_content();
                        }
                        
                        ui.add_space(10.0);
                    });
                    ui.add_space(10.0);
                });
            });
    }
}
