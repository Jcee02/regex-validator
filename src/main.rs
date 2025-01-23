use eframe::egui;
use egui::ViewportBuilder;
use regex::Regex;
use std::collections::HashMap;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Regex Validator",
        options,
        Box::new(|_cc| Box::new(RegexApp::default())),
    )
}

const ACCENT_COLOR: egui::Color32 = egui::Color32::from_rgb(70, 130, 180);
const SUCCESS_COLOR: egui::Color32 = egui::Color32::from_rgb(50, 205, 50);
const ERROR_COLOR: egui::Color32 = egui::Color32::from_rgb(220, 20, 60);
const BUTTON_PADDING: f32 = 5.0;
const SPACING: f32 = 10.0;

struct RegexApp {
    pattern: String,
    test_strings: Vec<String>,
    compiled_regex: Result<Regex, regex::Error>,
    new_test_string: String,
    presets: HashMap<String, String>,
}

impl Default for RegexApp {
    fn default() -> Self {
        let mut presets = HashMap::new();
        presets.insert("Email".to_string(), r"^[\w.-]+@([\w-]+\.)+[\w-]{2,4}$".to_string());
        presets.insert("Phone (US)".to_string(), r"^\+?1?\d{10}$".to_string());
        presets.insert("URL".to_string(), r"^https?://[\w\-\.]+(:\d+)?(/[\w/_\.\-]*)?$".to_string());
        
        Self {
            pattern: String::new(),
            test_strings: Vec::new(),
            compiled_regex: Ok(Regex::new("").unwrap()),
            new_test_string: String::new(),
            presets,
        }
    }
}

impl eframe::App for RegexApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update regex when pattern changes
        if let Ok(regex) = Regex::new(&self.pattern) {
            self.compiled_regex = Ok(regex);
        } else if !self.pattern.is_empty() {
            self.compiled_regex = Regex::new(&self.pattern);
        }

        // Top panel for regex input
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(SPACING);
            ui.horizontal(|ui| {
                ui.add_space(SPACING);
                ui.heading("Pattern: ");
                let pattern_edit = egui::TextEdit::singleline(&mut self.pattern)
                    .desired_width(300.0)
                    .hint_text("Enter regex pattern here...")
                    .show(ui);
                pattern_edit.response.on_hover_text("Enter your regular expression pattern here");
                
                ui.add_space(SPACING);
                // Show compilation status
                match &self.compiled_regex {
                    Ok(_) => {
                        ui.colored_label(SUCCESS_COLOR, "✓ Valid Regex");
                    }
                    Err(e) => {
                        ui.colored_label(ERROR_COLOR, format!("⚠ Invalid: {}", e));
                    }
                }
            });
        });

        // Side panel for presets
        egui::SidePanel::right("presets_panel").max_width(200.0).show(ctx, |ui| {
            ui.add_space(SPACING);
            ui.heading("Preset Patterns");
            ui.add_space(SPACING);
            egui::Frame::none()
                .fill(ui.visuals().extreme_bg_color)
                .inner_margin(8.0)
                .show(ui, |ui| {
                    for (name, pattern) in &self.presets {
                        let btn = ui.add_sized(
                            [180.0, 30.0],
                            egui::Button::new(name).fill(ACCENT_COLOR)
                        );
                        if btn.clicked() {
                            self.pattern = pattern.clone();
                        }
                        btn.on_hover_ui(|ui| {
                            ui.label(format!("Pattern: {}", pattern));
                        });
                        ui.add_space(4.0);
                    }
                });
        });

        // Central panel for test strings
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(SPACING);
            egui::Frame::none()
                .fill(ui.visuals().extreme_bg_color)
                .inner_margin(10.0)
                .show(ui, |ui| {
                    // Input for new test string
                    ui.horizontal(|ui| {
                        ui.add_space(SPACING);
                        ui.label("Test string: ");
                        let response = egui::TextEdit::singleline(&mut self.new_test_string)
                            .desired_width(300.0)
                            .hint_text("Enter test string here...")
                            .show(ui);
                        
                        let add_btn = ui.add_sized(
                            [60.0, 24.0],
                            egui::Button::new("Add").fill(ACCENT_COLOR)
                        );
                        
                        if add_btn.clicked() || (response.response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))) {
                            if !self.new_test_string.is_empty() {
                                self.test_strings.push(self.new_test_string.clone());
                                self.new_test_string.clear();
                            }
                        }
                        add_btn.on_hover_text("Add a new test string");
                    });

                    ui.add_space(SPACING);
                    ui.separator();
                    ui.add_space(SPACING);

                    // List of test strings with results
                    let mut to_remove = None;
                    egui::Frame::none()
                        .fill(ui.visuals().extreme_bg_color)
                        .show(ui, |ui| {
                            for (idx, test_string) in self.test_strings.iter().enumerate() {
                                ui.horizontal(|ui| {
                                    ui.add_space(SPACING);
                                    let remove_btn = ui.add_sized(
                                        [24.0, 24.0],
                                        egui::Button::new("❌")
                                    );
                                    if remove_btn.clicked() {
                                        to_remove = Some(idx);
                                    }
                                    remove_btn.on_hover_text("Remove this test string");
                                    
                                    let matches = self.compiled_regex.as_ref().map_or(false, |r| r.is_match(test_string));
                                    let color = if matches { SUCCESS_COLOR } else { ERROR_COLOR };
                                    
                                    ui.add_space(SPACING);
                                    ui.label(test_string);
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        ui.colored_label(color, if matches { "✓ Match" } else { "✗ No Match" });
                                    });
                                });
                                if idx < self.test_strings.len() - 1 {
                                    ui.add_space(4.0);
                                }
                            }
                        });

                    // Remove marked string
                    if let Some(idx) = to_remove {
                        self.test_strings.remove(idx);
                    }
                });
        });
    }
}
