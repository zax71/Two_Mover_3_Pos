use std::sync::{Arc, Mutex};

use crate::{app::GlobalState, light::Light};

#[derive(Default)]
pub struct OutputSection {
    toggleable_lights: Arc<Mutex<Vec<ToggleableLight>>>,
    global_state: Arc<GlobalState>,
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
    pub fn init(&mut self) {
        // Unwrap things from arcs etc
        let lights_ref = self.toggleable_lights.clone();
        let conn = self.global_state.database.connection.lock().unwrap();
        let global_state = self.global_state.clone();

        // Init the value of toggleable lights from the db
        self.toggleable_lights = Arc::new(Mutex::new(OutputSection::get_toggleable_lights(
            global_state.clone(),
        )));

        // Called whenever the db is updated
        conn.update_hook(Some(move |_, database: &str, table: &str, _| {
            if database != "main" && table != "Lights" {
                return;
            }

            let new_lights = OutputSection::get_toggleable_lights(global_state.clone());

            if let Ok(mut lights) = lights_ref.lock() {
                *lights = new_lights;
            }
        }));
    }

    fn get_toggleable_lights(global_state: Arc<GlobalState>) -> Vec<ToggleableLight> {
        let lights = global_state
            .database
            .get_lights()
            .expect("Failed to get lights from db");

        lights.iter().map(ToggleableLight::from_light).collect()
    }

    pub fn add(&mut self, ui: &mut egui::Ui) {
        ui.heading("Output settings");
        ui.collapsing("Select light", |ui| {
            self.toggleable_lights
                .lock()
                .unwrap()
                .iter_mut()
                .for_each(|light| {
                    ui.checkbox(&mut light.state, &light.light.name);
                });
        });
    }
}
