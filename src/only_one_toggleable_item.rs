use crate::components::toggleable_item::ToggleableItem;

/// This function will modify current_paths so that it only has one selected - preferring the newer of the two items if there are two items selected
pub fn only_one_toggleable_item<T>(
    current_items: &mut [ToggleableItem<T>], // Pass by reference and modify in place
    previous_items: Vec<ToggleableItem<T>>,
) {
    // Find the index that was selected before. Only 1 should be selected so exit early when we find it
    let mut previous_selection: Option<usize> = None;
    for (i, path) in previous_items.iter().enumerate() {
        if !path.state {
            continue;
        }

        previous_selection = Some(i);
        break;
    }

    // Loop through new paths, if we find that there are two items selected then pick the newer one
    let mut current_selection: Vec<usize> = vec![];
    for (i, path) in current_items.iter().enumerate() {
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
        Some(previous_selection) => current_selection.retain(|value| *value == previous_selection),
        None => return,
    }

    // Return the previous value to false
    current_items[current_selection[0]].state = false;
}

#[cfg(test)]
mod tests {

    use crate::components::toggleable_item::ToggleableItem;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_radio_zero_previous_true() {
        let previous_state = vec![
            ToggleableItem::from(1),
            ToggleableItem::from(2),
            ToggleableItem::from(3),
            ToggleableItem::from(4),
        ];

        let mut current_state = previous_state.clone();

        only_one_toggleable_item(&mut current_state, previous_state.clone());
        assert_eq!(current_state.clone(), previous_state);
    }

    #[test]
    fn test_radio_one_previous_true() {
        let mut previous_state = vec![
            ToggleableItem::from(1),
            ToggleableItem::from(2),
            ToggleableItem::from(3),
            ToggleableItem::from(4),
        ];

        previous_state[0].state = true;

        let mut current_state = previous_state.clone();

        only_one_toggleable_item(&mut current_state, previous_state.clone());
        assert_eq!(current_state.clone(), previous_state);
    }

    #[test]
    fn test_radio_one_previous_true_two_current_true() {
        let mut previous_state = vec![
            ToggleableItem::from(1),
            ToggleableItem::from(2),
            ToggleableItem::from(3),
            ToggleableItem::from(4),
        ];

        previous_state[0].state = true;

        let mut current_state = previous_state.clone();
        current_state[2].state = true;

        let mut expected_state = vec![
            ToggleableItem::from(1),
            ToggleableItem::from(2),
            ToggleableItem::from(3),
            ToggleableItem::from(4),
        ];
        expected_state[2].state = true;

        only_one_toggleable_item(&mut current_state, previous_state.clone());
        assert_eq!(current_state.clone(), expected_state);
    }
}
