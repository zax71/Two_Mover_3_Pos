use vector3d::Vector3d;

use crate::{
    app::GlobalState,
    components::{
        output_section::move_calculator, select_lights_modal::SelectLightsModal, select_vec,
    },
};

#[derive(Debug)]
pub struct DebugPointAt {
    point: Vector3d<f64>,
    pub shown: bool,
    select_lights_modal: SelectLightsModal,
}

impl DebugPointAt {
    pub fn new() -> Self {
        Self {
            point: Vector3d::default(),
            shown: false,
            select_lights_modal: SelectLightsModal::new("Select lights for debug point at"),
        }
    }
    pub fn add(&mut self, ctx: &egui::Context, app_state: &mut GlobalState) {
        self.select_lights_modal.add(ctx);

        egui::Window::new("Debug point at")
            .collapsible(false)
            .resizable(true)
            .fade_in(true)
            .fade_out(true)
            .open(&mut self.shown)
            .show(ctx, |ui| {
                if ui
                    .button("Select lights")
                    .on_hover_text("What lights should be used in this move?")
                    .clicked()
                {
                    self.select_lights_modal.update_lights(app_state);
                    self.select_lights_modal.shown = true
                }

                select_vec(ui, "Position: ", &mut self.point);

                if ui.button("Output OSC").clicked() {
                    let mut commands: Vec<String> = vec![];
                    for light in self.select_lights_modal.get_selected_lights() {
                        commands.append(&mut light.point_at(self.point).to_commands());
                    }

                    let osc_result = move_calculator::output_commands(commands, app_state);

                    match osc_result {
                        Ok(_) => (),
                        Err(err) => {
                            app_state
                                .toasts
                                .error(format!("Failed to send OSC command to desk: {err}"));
                        }
                    }
                }
            });
    }
}
