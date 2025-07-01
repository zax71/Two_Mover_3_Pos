use egui::Layout;

use crate::{app::GlobalState, components::select_vec, path::line::NamedLine};

pub struct AddLineWindow {
    current_line: NamedLine,
    pub shown: bool,
}

impl AddLineWindow {
    pub fn new() -> Self {
        Self {
            current_line: NamedLine::default(),
            shown: false,
        }
    }
    /// Writes the line to the specified Database
    fn save(app_state: &mut GlobalState, adding_line: &NamedLine) {
        let db_result = app_state.database.add_line(adding_line);

        match db_result {
            Ok(_) => {
                app_state.toasts.success(format!(
                    "Successfully added a new line: {}",
                    adding_line.name
                ));
            }

            Err(err) => {
                app_state
                    .toasts
                    .error(format!("Failed to write line to database: {}", err));
            }
        }
    }

    /// Adds the light entry window to the UI. Must be shown by setting `Self.shown = true`
    pub fn add(&mut self, ctx: &egui::Context, app_state: &mut GlobalState) {
        let mut open = self.shown;

        egui::Window::new("Add line")
            .collapsible(false)
            .resizable(true)
            .fade_in(true)
            .fade_out(true)
            .open(&mut self.shown)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Name:");
                        ui.text_edit_singleline(&mut self.current_line.name);
                    });
                    select_vec(ui, "Start position: ", &mut self.current_line.line.start);
                    select_vec(ui, "End position: ", &mut self.current_line.line.end);
                });
                ui.add_space(16.0);

                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        open = false;
                    }
                    ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Add").clicked() {
                            open = false;
                            Self::save(app_state, &self.current_line);
                        }
                    });
                })
            });

        // Support using the close button defined with `.open()` above
        if self.shown {
            self.shown = open;
        }
    }
}
