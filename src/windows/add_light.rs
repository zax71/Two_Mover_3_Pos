use eframe::egui::Ui;

use crate::app_component::AppComponent;
use crate::light::Light;
use crate::App;

pub struct AddLight {
    current_light: Light,
}

impl AppComponent for AddLight {
    type Context = App;

    fn add(ctx: &mut Self::Context, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label("Position:");
                ui.text_edit_singleline();
                ui.text_edit_singleline();
                ui.text_edit_singleline();
            });

            ui.horizontal(|ui| {
                ui.label("Name:");
                ui.text_edit_singleline();
            });

            ui.horizontal(|ui| {
                ui.label("Address:");
                ui.text_edit_singleline();
            });
        });
    }
}
