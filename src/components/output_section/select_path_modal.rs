use crate::{
    app::GlobalState,
    components::output_section::toggleable_item::ToggleableItem,
    path::{Path, PathEnum},
};

pub struct SelectPathModal {
    pub shown: bool,
    toggleable_paths: Vec<ToggleableItem<PathEnum>>,
}

impl SelectPathModal {
    pub fn new() -> Self {
        Self {
            shown: false,
            toggleable_paths: Vec::default(),
        }
    }

    pub fn update_paths(&mut self, app_state: &mut GlobalState) {
        let paths = match app_state.database.get_paths() {
            Ok(lines) => lines,
            Err(e) => {
                app_state.toasts.error(e.to_string());
                return;
            }
        };

        //Turn the Vec<PathEnum> into Vec<ToggleableItem<PathEnum>>
        let new_toggleable_paths: Vec<ToggleableItem<PathEnum>> =
            paths.iter().map(ToggleableItem::from_item).collect();

        // Delete paths that have been removed from the Vec
        for (i, path) in self.toggleable_paths.clone().into_iter().enumerate() {
            if !new_toggleable_paths.contains(&path) {
                self.toggleable_paths.remove(i);
            }
        }

        // Add new lights, if the len is less or equal then we have all the lights already
        if new_toggleable_paths.len() <= self.toggleable_paths.len() {
            return;
        }

        for path in new_toggleable_paths {
            if !self.toggleable_paths.contains(&path) {
                self.toggleable_paths.push(path.clone());
            }
        }
    }

    pub fn add(&mut self, ctx: &egui::Context) {
        egui::Window::new("Select Paths")
            .collapsible(false)
            .resizable(true)
            .fade_in(true)
            .fade_out(true)
            .open(&mut self.shown)
            .show(ctx, |ui| {
                for toggleable_light in &mut self.toggleable_paths {
                    ui.checkbox(&mut toggleable_light.state, &toggleable_light.item.name());
                }
            });
    }
}
