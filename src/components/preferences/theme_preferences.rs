use std::net::IpAddr;

use crate::components::preferences::PreferenceItem;

#[derive(Debug, Clone, PartialEq)]
pub struct ThemePreferences {}

impl ThemePreferences {
    pub fn new() -> Self {
        Self {}
    }
}

impl PreferenceItem for ThemePreferences {
    fn show(&mut self, ui: &mut egui::Ui, global_state: &mut crate::app::GlobalState) {
        ui.label("Theme Preferences");
        egui::widgets::global_theme_preference_buttons(ui);
    }

    fn name(&self) -> &str {
        "Theme"
    }
}
