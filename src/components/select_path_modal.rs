use crate::{
    app::GlobalState,
    components::toggleable_item::ToggleableItem,
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

    /// Returns the currently selected path. If none are selected then the Option value will be None.
    pub fn get_selected_path(&self) -> Option<PathEnum> {
        for current_path in &self.toggleable_paths {
            if current_path.state {
                return Some(current_path.unwrap());
            }
        }

        return None;
    }

    /// This function will modify current_paths so that it only has one selected - preferring the newer of the two items if there are two items selected
    fn only_one_radio<T>(
        current_paths: &mut [ToggleableItem<T>], // Pass by reference and modify in place
        previous_paths: Vec<ToggleableItem<T>>,
    ) {
        // Find the index that was selected before. Only 1 should be selected so exit early when we find it
        let mut previous_selection: Option<usize> = None;
        for (i, path) in previous_paths.iter().enumerate() {
            if !path.state {
                continue;
            }

            previous_selection = Some(i);
            break;
        }

        // Loop through new paths, if we find that there are two items selected then pick the newer one
        let mut current_selection: Vec<usize> = vec![];
        for (i, path) in current_paths.iter().enumerate() {
            if !path.state {
                continue;
            }

            current_selection.push(i);
        }

        // Check that len is not > 2 as that would indicate something weird is happening
        if current_selection.len() > 2 {
            panic!("There are more than 2 selected items in the select path modal!")
        }

        // Do nothing if there is only one thing selected
        if current_selection.len() == 1 {
            return;
        }

        // If two items are selected, we need to remove the older one
        match previous_selection {
            Some(previous_selection) => {
                current_selection.retain(|value| *value == previous_selection)
            }
            None => return,
        }

        // Return the previous value to false
        current_paths[current_selection[0]].state = false;
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
                Self::only_one_radio(&mut self.toggleable_paths, previous_paths);
            });
    }
}

#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_radio_zero_previous_true() {
        let previous_state = vec![
            ToggleableItem::from_item(&1),
            ToggleableItem::from_item(&2),
            ToggleableItem::from_item(&3),
            ToggleableItem::from_item(&4),
        ];

        let mut current_state = previous_state.clone();

        SelectPathModal::only_one_radio(&mut current_state, previous_state.clone());
        assert_eq!(current_state.clone(), previous_state);
    }

    #[test]
    fn test_radio_one_previous_true() {
        let mut previous_state = vec![
            ToggleableItem::from_item(&1),
            ToggleableItem::from_item(&2),
            ToggleableItem::from_item(&3),
            ToggleableItem::from_item(&4),
        ];

        previous_state[0].state = true;

        let mut current_state = previous_state.clone();

        SelectPathModal::only_one_radio(&mut current_state, previous_state.clone());
        assert_eq!(current_state.clone(), previous_state);
    }

    #[test]
    fn test_radio_one_previous_true_two_current_true() {
        let mut previous_state = vec![
            ToggleableItem::from_item(&1),
            ToggleableItem::from_item(&2),
            ToggleableItem::from_item(&3),
            ToggleableItem::from_item(&4),
        ];

        previous_state[0].state = true;

        let mut current_state = previous_state.clone();
        current_state[2].state = true;

        let mut expected_state = vec![
            ToggleableItem::from_item(&1),
            ToggleableItem::from_item(&2),
            ToggleableItem::from_item(&3),
            ToggleableItem::from_item(&4),
        ];
        expected_state[2].state = true;

        SelectPathModal::only_one_radio(&mut current_state, previous_state.clone());
        assert_eq!(current_state.clone(), expected_state);
    }
}
