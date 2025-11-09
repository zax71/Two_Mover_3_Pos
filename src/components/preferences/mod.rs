use egui::ScrollArea;
use enum_dispatch::enum_dispatch;

use crate::{
    app::GlobalState,
    components::{
        preferences::{osc_preferences::OscPreferences, theme_preferences::ThemePreferences},
        toggleable_item::ToggleableItem,
    },
    only_one_toggleable_item::only_one_toggleable_item,
};

pub mod osc_preferences;
pub mod theme_preferences;

#[enum_dispatch]
#[derive(Debug, Clone, PartialEq)]
pub enum PreferenceItemEnum {
    OscPreferences,
    ThemePreferences,
}

/// A window in the Preferences modal
#[enum_dispatch(PreferenceItemEnum)]
pub trait PreferenceItem {
    fn show(&mut self, ui: &mut egui::Ui, global_state: &mut GlobalState);
    fn update(&mut self, global_state: &mut crate::app::GlobalState);
    fn name(&self) -> &str;
}

#[derive(Debug, Default)]
pub struct Preferences {
    shown: bool,
    preference_items: Vec<ToggleableItem<PreferenceItemEnum>>,
}

impl Preferences {
    pub fn new() -> Self {
        Self {
            shown: false,
            preference_items: vec![
                ToggleableItem::from(PreferenceItemEnum::OscPreferences(OscPreferences::default())),
                ToggleableItem::from(PreferenceItemEnum::ThemePreferences(ThemePreferences::new())),
            ],
        }
    }

    /// Shows this UI element & updates internal state
    pub fn show(&mut self, global_state: &mut GlobalState) {
        self.shown = true;
        for preference_item in &mut self.preference_items {
            preference_item.item.update(global_state);
        }
    }

    pub fn add(&mut self, ctx: &egui::Context, global_state: &mut GlobalState) {
        let mut open = self.shown;
        egui::Window::new("Preferences")
            .collapsible(false)
            .resizable(true)
            .fade_in(true)
            .fade_out(true)
            .open(&mut open)
            .show(ctx, |ui| {
                let old_preferences = self.preference_items.clone();

                egui::SidePanel::left("preferences_list")
                    .resizable(true)
                    .default_width(100.0)
                    .min_width(100.0)
                    // I'm using .show_inside() instead of .show() to draw this in something other than the root window
                    .show_inside(ui, |ui| self.sidebar_content(ui));

                for (i, preference_item) in self.preference_items.iter_mut().enumerate() {
                    if preference_item.state {
                        // Update state in preference item if this is first time opening it. Not doing each frame due to file I/O
                        if old_preferences[i].state == false {
                            println!("First time opening");
                            preference_item.item.update(global_state);
                        }
                        preference_item.item.show(ui, global_state);
                    }
                }

                // Ensure that only one preference item is selected
                only_one_toggleable_item(&mut self.preference_items, old_preferences);
            });

        self.shown = open;
    }

    /// The sidebar menu's content: toggleable values aligned vertically in a scrollable area
    fn sidebar_content(&mut self, ui: &mut egui::Ui) {
        ScrollArea::vertical().show(ui, |ui| {
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                for preference_item in &mut self.preference_items {
                    ui.toggle_value(&mut preference_item.state, preference_item.item.name());
                }
            });
        });
    }
}
