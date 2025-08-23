use crate::{app::GlobalState, light::Light};

pub struct SelectLightsModal {
    pub shown: bool,
    toggleable_lights: Vec<ToggleableLight>,
}

#[derive(Default)]
struct ToggleableLight {
    light: Light,
    state: bool,
}

impl ToggleableLight {
    pub fn from_light(light: &Light) -> Self {
        Self {
            light: light.clone(),
            state: false,
        }
    }
}

impl SelectLightsModal {
    pub fn new() -> Self {
        Self {
            shown: false,
            toggleable_lights: Vec::default(),
        }
    }

    pub fn update_lights(&mut self, app_state: &mut GlobalState) {
        let lights = match app_state.database.get_lights() {
            Ok(lights) => lights,
            Err(e) => {
                app_state.toasts.error(e.to_string());
                return;
            }
        };
        self.toggleable_lights = lights.iter().map(ToggleableLight::from_light).collect();
    }

    pub fn add(&mut self, ctx: &egui::Context) {
        egui::Window::new("Select Lights")
            .collapsible(false)
            .resizable(true)
            .fade_in(true)
            .fade_out(true)
            .open(&mut self.shown)
            .show(ctx, |ui| {
                for toggleable_light in &mut self.toggleable_lights {
                    ui.checkbox(&mut toggleable_light.state, &toggleable_light.light.name);
                }

                ui.vertical_centered(|ui| {
                    if ui
                        .button("Save")
                        .on_hover_text(
                            "Saves your selection, press the \"x\" to close without saving",
                        )
                        .clicked()
                    {
                        todo!("Save the selected lights");
                    }
                });
            });
    }
}
