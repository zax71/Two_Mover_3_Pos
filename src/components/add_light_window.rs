use egui::{DragValue, Layout};

use crate::{app::GlobalState, components::select_vec, light::Light};

pub struct AddLightWindow {
    current_light: Light,
    pub shown: bool,
}

impl AddLightWindow {
    pub fn new() -> Self {
        Self {
            current_light: Light::default(),
            shown: false,
        }
    }
    /// Writes the light to the specified Database
    fn save(app_state: &mut GlobalState, adding_light: &Light) {
        let db_result = app_state.database.add_light(adding_light);

        match db_result {
            Ok(_) => {
                app_state
                    .toasts
                    .success(format!("Successfully added light {}", adding_light.name));
            }

            Err(err) => {
                app_state
                    .toasts
                    .error(format!("Failed to write light to database: {err}"));
            }
        }
    }

    /// Adds the light entry window to the UI. Must be shown by setting `Self.shown = true`
    pub fn add(&mut self, ctx: &egui::Context, app_state: &mut GlobalState) {
        let mut open = self.shown;

        egui::Window::new("Add light")
            .collapsible(false)
            .resizable(true)
            .fade_in(true)
            .fade_out(true)
            .open(&mut self.shown)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    select_vec(ui, "Position: ", &mut self.current_light.coordinates);

                    ui.horizontal(|ui| {
                        ui.label("Name:");
                        ui.text_edit_singleline(&mut self.current_light.name);
                    });

                    ui.horizontal(|ui| {
                        ui.label("Address:");
                        ui.add(DragValue::new(&mut self.current_light.address));
                    });
                });
                ui.add_space(16.0);

                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        open = false;
                    }
                    ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Add").clicked() {
                            open = false;
                            Self::save(app_state, &self.current_light);
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
