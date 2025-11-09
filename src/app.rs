use std::fs;

use egui_notify::Toasts;
use measurements::data;

use crate::components::add_light_window::AddLightWindow;
use crate::components::add_path::add_bezier::AddBezierWindow;
use crate::components::add_path::add_cubic_bezier::AddCubicBezierWindow;
use crate::components::add_path::add_line_window::AddLineWindow;
use crate::components::debug_point_at::DebugPointAt;
use crate::components::output_section::OutputSection;
use crate::components::preferences::Preferences;
use crate::config::ConfigFile;
use crate::db::Database;

pub struct GlobalState {
    pub database: Database,
    pub config_file: ConfigFile,
    pub toasts: Toasts,
}

impl Default for GlobalState {
    fn default() -> Self {
        let mut data_path = dirs::data_dir().expect("Could not find OS data directory");
        data_path.push("two_mover_3_pos");

        fs::create_dir_all(&data_path).expect("Failed to create directories for database");

        let mut db_path = data_path.clone();
        db_path.push("database");
        db_path.set_extension("db");

        let mut config_path = data_path.clone();
        config_path.push("config");
        config_path.set_extension("toml");
        Self {
            database: Database::new(db_path),
            config_file: ConfigFile::new(config_path)
                .expect("Failed to create and/or read config file"),
            toasts: Toasts::default(),
        }
    }
}

pub struct App {
    add_light_window: AddLightWindow,
    add_line_window: AddLineWindow,
    add_bezier_window: AddBezierWindow,
    add_cubic_bezier_window: AddCubicBezierWindow,
    debug_point_at: DebugPointAt,
    output_section: OutputSection,
    preferences: Preferences,
    global_state: GlobalState,
}
impl Default for App {
    /// Initialize the `App` struct with it's default values
    fn default() -> Self {
        let global_state = GlobalState::default();

        Self {
            add_light_window: AddLightWindow::new(),
            add_line_window: AddLineWindow::new(),
            add_bezier_window: AddBezierWindow::new(),
            add_cubic_bezier_window: AddCubicBezierWindow::new(),
            debug_point_at: DebugPointAt::new(),
            output_section: OutputSection::new(),
            preferences: Preferences::new(),
            global_state,
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }

    fn menu_bar(&mut self, ui: &mut egui::Ui) {
        egui::MenuBar::new().ui(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Quit").clicked() {
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                }

                ui.menu_button("Debug", |ui| {
                    if ui.button("Light point at").clicked() {
                        self.debug_point_at.shown = true;
                        ui.close();
                    }
                })
            });

            ui.menu_button("Edit", |ui| {
                if ui.button("Preferences").clicked() {
                    self.preferences.show(&mut self.global_state);
                    ui.close();
                }
            });

            ui.menu_button("Theme", |ui| {
                egui::widgets::global_theme_preference_buttons(ui);
            });

            ui.menu_button("Add", |ui| {
                if ui.button("Light").clicked() {
                    self.add_light_window.shown = true;
                    ui.close();
                }

                ui.menu_button("Path", |ui| {
                    if ui.button("Line").clicked() {
                        self.add_line_window.shown = true;
                        ui.close();
                    }
                    if ui.button("Bezier curve").clicked() {
                        self.add_bezier_window.shown = true;
                        ui.close();
                    }
                    if ui.button("Cubic Bezier").clicked() {
                        self.add_cubic_bezier_window.shown = true;
                        ui.close();
                    }
                });
            });
        });
    }
}

impl eframe::App for App {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        // Show the windows
        self.add_light_window.add(ctx, &mut self.global_state);
        self.add_line_window.add(ctx, &mut self.global_state);
        self.add_bezier_window.add(ctx, &mut self.global_state);
        self.add_cubic_bezier_window
            .add(ctx, &mut self.global_state);
        self.debug_point_at.add(ctx, &mut self.global_state);
        self.preferences.add(ctx, &mut self.global_state);

        // Show toasts
        self.global_state.toasts.show(ctx);

        // Show menu bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            self.menu_bar(ui);
        });

        // Add output section
        egui::SidePanel::right("output").show(ctx, |ui| {
            self.output_section.add(ctx, ui, &mut self.global_state);
        });

        // Boast about being written in egui
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
