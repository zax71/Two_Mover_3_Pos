use std::fs;
use std::ops::Add;

use crate::components::add_light_window::AddLightWindow;
use crate::db::{self, Database};
use crate::light::Light;

#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    pub current_light: Light,
    add_light_window: AddLightWindow,

    #[serde(skip)]
    database: Database,
}

impl Default for App {
    /// Initialize the `App` struct with it's default values
    fn default() -> Self {
        let mut db_path = dirs::data_dir().expect("Could not find OS data directory");
        db_path.push("two_mover_3_pos");

        fs::create_dir_all(&db_path).expect("Failed to create directories for database");

        db_path.push("database");
        db_path.set_extension("db");

        Self {
            current_light: Light::default(),
            add_light_window: AddLightWindow::default(),
            database: Database::new(db_path),
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for App {

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.menu_button("Theme", |ui| {
                    egui::widgets::global_theme_preference_buttons(ui);
                });

                ui.menu_button("Add", |ui| {
                    if ui.button("Light").clicked() {
                        self.add_light_window.shown = true
                    }

                    if ui.button("Path").clicked() {
                        todo!("Implement adding paths")
                    }
                });
            });
        });

        // Show the add light window
        self.add_light_window.add(ctx);

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
