use egui::DragValue;

use crate::app::GlobalState;
use crate::components::select_lights_modal::SelectLightsModal;
use crate::components::select_path_modal::SelectPathModal;

pub mod move_calculator;

pub struct OutputSection {
    select_lights_modal: SelectLightsModal,
    select_path_modal: SelectPathModal,
    selected_output_type: OutputType,
    move_time: f64,
    frames: u16,
    cue_number: u32,
}

#[derive(Debug, PartialEq)]
enum OutputType {
    Osc,
    Instructions,
}

impl OutputSection {
    pub fn new() -> Self {
        Self {
            select_lights_modal: SelectLightsModal::new("Select lights for move"),
            select_path_modal: SelectPathModal::new(),
            selected_output_type: OutputType::Instructions,
            move_time: 1.0,
            frames: 10,
            cue_number: 1,
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
                ui.add(DragValue::new(&mut self.move_time).speed(0.1));
                ui.label("s");
            });

            ui.horizontal(|ui| {
                ui.label("Frames");
                ui.add(DragValue::new(&mut self.frames));
            });

            ui.horizontal(|ui| {
                ui.label("Cue Number");
                ui.add(DragValue::new(&mut self.cue_number));
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
                self.execute_move(app_state);
            }

            // Ensure that move time is never negative. Time travel doesn't exist!
            if self.move_time < 0.0 {
                self.move_time = 0.0
            }

            // If there are more than 99 frames then there's no way to number then in EOS with the current system I'm using.
            // It makes no sense to have 0 frames
            self.frames = self.frames.clamp(1, 99);

            // There's no cue 0 in EOS
            if self.cue_number == 0 {
                self.cue_number = 1
            }
        });
    }

    fn execute_move(&self, app_state: &mut GlobalState) {
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

        if lights.is_empty() {
            app_state
                .toasts
                .warning("No lights are selected - try selecting some lights");
            return;
        }

        let frames = move_calculator::calculate_move(path, lights, self.frames, self.move_time);
        let commands = move_calculator::frames_to_commands(frames, self.cue_number);
        //println!("{:#?}", commands);
        // TODO: Remove this except
        move_calculator::output_commands(commands, app_state).expect("Failed to output commands");
    }
}
