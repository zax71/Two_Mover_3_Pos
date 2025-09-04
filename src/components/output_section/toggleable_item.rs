#[derive(Default, Clone)]
pub struct ToggleableItem<T> {
    pub item: T,
    pub state: bool,
}

impl<T: PartialEq> PartialEq for ToggleableItem<T> {
    /// Measures the equality of the toggleable light. Ignores the state of the light
    fn eq(&self, other: &Self) -> bool {
        self.item == other.item
    }
}

impl<T: Clone> ToggleableItem<T> {
    pub fn from_item(item: &T) -> Self {
        Self {
            item: item.clone(),
            state: false,
        }
    }
}
