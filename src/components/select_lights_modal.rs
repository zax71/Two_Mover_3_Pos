use crate::{app::GlobalState, components::toggleable_item::ToggleableItem, light::Light};

#[derive(Debug)]
pub struct SelectLightsModal {
    pub shown: bool,
    toggleable_lights: Vec<ToggleableItem<Light>>,
    pub title: String,
}

impl SelectLightsModal {
    pub fn new(title: &str) -> Self {
        Self {
            shown: false,
            toggleable_lights: Vec::default(),
            title: title.to_string(),
        }
    }

    /// This function adds any new lights to the UI that have been added since it's construction, while keeping the toggled state of any existing lights
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

    /// Gets the selected lights - can be an empty vec if no lights are selected
    pub fn get_selected_lights(&self) -> Vec<Light> {
        let mut lights: Vec<Light> = vec![];

        for current_light in &self.toggleable_lights {
            if current_light.state {
                lights.push(current_light.unwrap());
            }
        }

        lights
    }

    /// Draw the select lights modal to the UI - called every frame
    pub fn add(&mut self, ctx: &egui::Context) {
        egui::Window::new(self.title.clone())
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
