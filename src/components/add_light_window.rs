use egui::{DragValue, Layout};
use serde::{Deserialize, Serialize};

use crate::light::Light;

#[derive(Default, Serialize, Deserialize)]
pub struct AddLightWindow {
    current_light: Light,
    pub shown: bool,
}

impl AddLightWindow {
    /// Writes the light to the specified Database
    fn save() {
        todo!("Implement database")
    }

    /// Adds the light entry window to the UI. Must be shown by setting `Self.shown = true`
    pub fn add(&mut self, ctx: &egui::Context) {
        let mut open = self.shown;

        egui::Window::new("Add light")
            .collapsible(false)
            .resizable(true)
            .fade_in(true)
            .fade_out(true)
            .open(&mut self.shown)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Position:");
                        ui.add(DragValue::new(&mut self.current_light.coordinates.x).speed(0.1));
                        ui.add(DragValue::new(&mut self.current_light.coordinates.y).speed(0.1));
                        ui.add(DragValue::new(&mut self.current_light.coordinates.z).speed(0.1));
                    });

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
                            Self::save();
                        }
                    });
                })
            });

        self.shown = open;
    }
}
