use egui::Layout;

use crate::{app::GlobalState, components::select_vec, path::cubic_bezier::CubicBezier};

pub struct AddCubicBezierWindow {
    current_cubic_bezier: CubicBezier,
    pub shown: bool,
}

impl AddCubicBezierWindow {
    pub fn new() -> Self {
        Self {
            current_cubic_bezier: CubicBezier::default(),
            shown: false,
        }
    }
    /// Writes the line to the specified Database
    fn save(app_state: &mut GlobalState, adding_cubic_bezier: &CubicBezier) {
        let db_result = app_state.database.add_cubic_bezier(adding_cubic_bezier);

        match db_result {
            Ok(_) => {
                app_state.toasts.success(format!(
                    "Successfully added a new cubic Bezier curve: {}",
                    adding_cubic_bezier.name
                ));
            }

            Err(err) => {
                app_state.toasts.error(format!(
                    "Failed to write cubic Bezier curve to database: {err}"
                ));
            }
        }
    }

    /// Adds the light entry window to the UI. Must be shown by setting `Self.shown = true`
    pub fn add(&mut self, ctx: &egui::Context, app_state: &mut GlobalState) {
        let mut open = self.shown;

        egui::Window::new("Add Cubic Bezier")
            .collapsible(false)
            .resizable(true)
            .fade_in(true)
            .fade_out(true)
            .open(&mut self.shown)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Name:");
                        ui.text_edit_singleline(&mut self.current_cubic_bezier.name);
                    });
                    select_vec(ui, "Start position: ", &mut self.current_cubic_bezier.start);
                    select_vec(ui, "End position: ", &mut self.current_cubic_bezier.end);
                    select_vec(ui, "Handle 1: ", &mut self.current_cubic_bezier.handle_1);
                    select_vec(ui, "Handle 2: ", &mut self.current_cubic_bezier.handle_2);
                });
                ui.add_space(16.0);

                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        open = false;
                    }
                    ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Add").clicked() {
                            open = false;
                            Self::save(app_state, &self.current_cubic_bezier);
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
