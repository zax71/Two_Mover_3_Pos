use std::fs;

use egui_notify::Toasts;

use crate::components::add_bezier::AddBezierWindow;
use crate::components::add_cubic_bezier::AddCubicBezierWindow;
use crate::components::add_light_window::AddLightWindow;
use crate::components::add_line_window::AddLineWindow;
use crate::components::output_section::OutputSection;
use crate::db::Database;
use crate::light::Light;

pub struct GlobalState {
    pub database: Database,
    pub toasts: Toasts,
}

impl Default for GlobalState {
    fn default() -> Self {
        let mut db_path = dirs::data_dir().expect("Could not find OS data directory");
        db_path.push("two_mover_3_pos");

        fs::create_dir_all(&db_path).expect("Failed to create directories for database");

        db_path.push("database");
        db_path.set_extension("db");
        Self {
            database: Database::new(db_path),
            toasts: Toasts::default(),
        }
    }
}

pub struct App {
    pub current_light: Light,
    add_light_window: AddLightWindow,
    add_line_window: AddLineWindow,
    add_bezier_window: AddBezierWindow,
    add_cubic_bezier_window: AddCubicBezierWindow,
    output_section: OutputSection,
    global_state: GlobalState,
}
impl Default for App {
    /// Initialize the `App` struct with it's default values
    fn default() -> Self {
        let global_state = GlobalState::default();

        Self {
            current_light: Light::default(),
            add_light_window: AddLightWindow::new(),
            add_line_window: AddLineWindow::new(),
            add_bezier_window: AddBezierWindow::new(),
            add_cubic_bezier_window: AddCubicBezierWindow::new(),
            output_section: OutputSection::default(),
            global_state,
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut new_self = Self::default();

        println!("Initializing output section");
        new_self.output_section.init();

        new_self
    }

    fn menu_bar(&mut self, ui: &mut egui::Ui) {
        egui::MenuBar::new().ui(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Quit").clicked() {
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
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

        // Show toasts
        self.global_state.toasts.show(ctx);

        // Show menu bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            self.menu_bar(ui);
        });

        egui::SidePanel::right("output").show(ctx, |ui| {
            self.output_section.add(ui);
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
