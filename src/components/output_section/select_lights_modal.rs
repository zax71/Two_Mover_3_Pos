use crate::{
    app::GlobalState, components::output_section::toggleable_item::ToggleableItem, light::Light,
};

pub struct SelectLightsModal {
    pub shown: bool,
    toggleable_lights: Vec<ToggleableItem<Light>>,
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

        let new_toggleable_lights: Vec<ToggleableItem<Light>> =
            lights.iter().map(ToggleableItem::from_item).collect();

        // Delete lights that have been removed from the Vec
        for (i, light) in self.toggleable_lights.clone().into_iter().enumerate() {
            if !new_toggleable_lights.contains(&light) {
                self.toggleable_lights.remove(i);
            }
        }

        // Add new lights, if the len is less or equal then we have all the lights already
        if new_toggleable_lights.len() <= self.toggleable_lights.len() {
            return;
        }

        for light in new_toggleable_lights {
            if !self.toggleable_lights.contains(&light) {
                self.toggleable_lights.push(light.clone());
            }
        }
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
                    ui.checkbox(&mut toggleable_light.state, &toggleable_light.item.name);
                }
            });
    }
}
