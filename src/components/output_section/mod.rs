use egui::DragValue;

use crate::{
    app::GlobalState,
    components::output_section::{
        select_lights_modal::SelectLightsModal, select_path_modal::SelectPathModal,
    },
};

mod move_calculator;
mod select_lights_modal;
mod select_path_modal;
mod toggleable_item;

pub struct OutputSection {
    select_lights_modal: SelectLightsModal,
    select_path_modal: SelectPathModal,
    selected_output_type: OutputType,
    move_time: f64,
    frames: u16,
}

#[derive(Debug, PartialEq)]
enum OutputType {
    Osc,
    Instructions,
}

impl OutputSection {
    pub fn new() -> Self {
        Self {
            select_lights_modal: SelectLightsModal::new(),
            select_path_modal: SelectPathModal::new(),
            selected_output_type: OutputType::Instructions,
            move_time: 1.0,
            frames: 10,
        }
    }

    pub fn add(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, app_state: &mut GlobalState) {
        // Add modals
        self.select_lights_modal.add(ctx);
        self.select_path_modal.add(ctx);

        ui.vertical_centered(|ui| {
            ui.heading("Output settings");
            if ui
                .button("Select lights")
                .on_hover_text("What lights should be used in this move?")
                .clicked()
            {
                self.select_lights_modal.update_lights(app_state);
                self.select_lights_modal.shown = true
            }

            if ui
                .button("Select path")
                .on_hover_text("What path should these lights move along?")
                .clicked()
            {
                self.select_path_modal.update_paths(app_state);
                self.select_path_modal.shown = true
            }

            ui.horizontal(|ui| {
                ui.label("Move time");
                ui.add(DragValue::new(&mut self.move_time));
                ui.label("s");
            });

            ui.horizontal(|ui| {
                ui.label("Frames");
                ui.add(DragValue::new(&mut self.frames));
            });

            egui::ComboBox::from_label("Output Type")
                .selected_text(format!("{:?}", self.selected_output_type))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.selected_output_type, OutputType::Osc, "OSC");
                    ui.selectable_value(
                        &mut self.selected_output_type,
                        OutputType::Instructions,
                        "Instructions",
                    );
                });

            if ui.button("Execute move").clicked() {
                let some_paths = self.select_path_modal.get_selected_path();
                let lights = self.select_lights_modal.get_selected_lights();

                let path = match some_paths {
                    Some(path) => path,
                    None => {
                        app_state
                            .toasts
                            .warning("No path is selected - try selecting a path");
                        return;
                    }
                };

                if lights.len() == 0 {
                    app_state
                        .toasts
                        .warning("No lights are selected - try selecting some lights");
                    return;
                }

                let frames =
                    move_calculator::calculate_move(path, lights, self.frames, self.move_time);

                println!("{:#?}", frames)
            }
        });
    }
}
