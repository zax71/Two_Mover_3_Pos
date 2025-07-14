use crate::light::Light;

pub struct OutputSection {
    toggleable_lights: Vec<ToggleableLight>,
}

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

impl OutputSection {
    pub fn new(lights: Vec<Light>) -> Self {
        // Convert the Vec<Light> to Vec<ToggleableLight> with the toggle set to false
        let toggleable_lights = lights
            .iter()
            .map(|light| ToggleableLight::from_light(light))
            .collect();

        Self { toggleable_lights }
    }

    pub fn add(&mut self, ui: &mut egui::Ui) {
        ui.heading("Output settings");
        ui.collapsing("Select light", |ui| {
            for toggleable_light in &mut self.toggleable_lights {
                ui.checkbox(&mut toggleable_light.state, &toggleable_light.light.name);
            }
        });
    }
}
