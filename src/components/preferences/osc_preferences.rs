use std::net::{IpAddr, Ipv4Addr};

use crate::components::preferences::PreferenceItem;

#[derive(Debug, Clone, PartialEq)]
pub struct OscPreferences {
    host: (IpAddr, u16),
    desk: (IpAddr, u16),
}

impl OscPreferences {
    pub fn new() -> Self {
        Self {
            host: (IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0),
            desk: (IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0),
        }
    }
}

impl PreferenceItem for OscPreferences {
    fn show(&mut self, ui: &mut egui::Ui, global_state: &mut crate::app::GlobalState) {
        ui.label("OSC Preferences");
    }

    fn name(&self) -> &str {
        "🖧 OSC"
    }
}
