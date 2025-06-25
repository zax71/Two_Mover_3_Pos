use std::fs;

use egui_notify::Toasts;

use crate::components::add_light_window::AddLightWindow;
use crate::components::menu_bar::MenuBar;
use crate::db::Database;
use crate::light::Light;

pub struct GlobalState {
    pub database: Database,
    pub toasts: Toasts,
    pub add_light_window_status: bool,
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
            add_light_window_status: false,
        }
    }
}

pub struct App {
    pub current_light: Light,
    pub add_light_window: AddLightWindow,
    global_state: GlobalState,
}

impl Default for App {
    /// Initialize the `App` struct with it's default values
    fn default() -> Self {
        let global_state = GlobalState::default();

        Self {
            current_light: Light::default(),
            add_light_window: AddLightWindow::new(),
            global_state,
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for App {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        // Show menu bar
        MenuBar::ui(ctx, &mut self.global_state);

        // Show the add light window
        self.add_light_window.shown = self.global_state.add_light_window_status;
        self.add_light_window.ui(ctx, &mut self.global_state);

        // Show toasts
        self.global_state.toasts.show(ctx);

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
        ui.label("Made with ❤️ using");
        ui.hyperlink_to("Rust", "https://www.rust-lang.org/");
        ui.label(", ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
