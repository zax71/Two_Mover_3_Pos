use crate::{
    app::GlobalState,
    components::toggleable_item::ToggleableItem,
    only_one_toggleable_item::only_one_toggleable_item,
    path::{Path, PathEnum},
};

pub struct SelectPathModal {
    pub shown: bool,
    pub toggleable_paths: Vec<ToggleableItem<PathEnum>>,
}

impl SelectPathModal {
    pub fn new() -> Self {
        Self {
            shown: false,
            toggleable_paths: Vec::default(),
        }
    }

    /// This function adds any new paths to the UI that have been added since it's construction, while keeping the toggled state of any existing paths
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
            paths.into_iter().map(ToggleableItem::from).collect();

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

    /// Returns the currently selected path. If none are selected then the Option value will be None.
    pub fn get_selected_path(&self) -> Option<PathEnum> {
        for current_path in &self.toggleable_paths {
            if current_path.state {
                return Some(current_path.unwrap());
            }
        }

        None
    }

    /// This adds the select path modal to the UI, called every frame
    pub fn add(&mut self, ctx: &egui::Context) {
        egui::Window::new("Select Path")
            .collapsible(false)
            .resizable(true)
            .fade_in(true)
            .fade_out(true)
            .open(&mut self.shown)
            .show(ctx, |ui| {
                let previous_paths = self.toggleable_paths.clone();
                for toggleable_path in &mut self.toggleable_paths {
                    ui.radio_value(
                        &mut toggleable_path.state,
                        true,
                        toggleable_path.item.name(),
                    );
                }
                // Make sure that only one of the radio buttons is selected at a time
                only_one_toggleable_item(&mut self.toggleable_paths, previous_paths);
            });
    }
}
