use std::net::Ipv4Addr;

use egui::DragValue;
use serde::{Deserialize, Serialize};

use crate::components::preferences::PreferenceItem;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OscPreferences {
    pub host: (Ipv4Addr, u16),
    pub desk: (Ipv4Addr, u16),
}

impl Default for OscPreferences {
    fn default() -> Self {
        Self {
            host: (Ipv4Addr::new(0, 0, 0, 0), 0),
            desk: (Ipv4Addr::new(192, 168, 0, 0), 8000),
        }
    }
}

impl PreferenceItem for OscPreferences {
    fn show(&mut self, ui: &mut egui::Ui, global_state: &mut crate::app::GlobalState) {
        ui.vertical(|ui| {
            select_ip_port(ui, "Host", &mut self.host);
            select_ip_port(ui, "Desk", &mut self.desk);
        });

        // Save config on click, show error message if there are issues
        if ui.button("Save").clicked() {
            match global_state.config_file.write_osc(self.clone()) {
                Ok(_) => {
                    global_state
                        .toasts
                        .success("Successfully changed OSC addresses!");
                }
                Err(e) => {
                    global_state
                        .toasts
                        .error(format!("Failed to save OSC config to file: {e}"));
                }
            }
        }
    }

    fn name(&self) -> &str {
        "🖧 OSC"
    }

    /// To be called before opening this UI element to update it's data with the config file.
    /// Causes file IO so do **not** call on every frame
    fn update(&mut self, global_state: &mut crate::app::GlobalState) {
        let config = global_state.config_file.read();

        match config {
            Ok(config) => {
                self.desk = config.osc.desk;
                self.host = config.osc.host;
            }
            Err(e) => {
                global_state
                    .toasts
                    .error(format!("Failed to read config from file: {e}"));
            }
        }
    }
}

/// Shows a UI element to select an IP address
fn select_ipv4(ui: &mut egui::Ui, name: &str, selecting_ip: &mut Ipv4Addr) {
    let mut octet_0 = selecting_ip.octets()[0];
    let mut octet_1 = selecting_ip.octets()[1];
    let mut octet_2 = selecting_ip.octets()[2];
    let mut octet_3 = selecting_ip.octets()[3];

    ui.horizontal(|ui| {
        ui.label(name);
        ui.add(DragValue::new(&mut octet_0).speed(1));
        ui.label(".");
        ui.add(DragValue::new(&mut octet_1).speed(1));
        ui.label(".");
        ui.add(DragValue::new(&mut octet_2).speed(1));
        ui.label(".");
        ui.add(DragValue::new(&mut octet_3).speed(1));
    });

    *selecting_ip = Ipv4Addr::new(octet_0, octet_1, octet_2, octet_3);
}

/// Shows a UI element to select an IP address and port
fn select_ip_port(ui: &mut egui::Ui, name: &str, selecting_address: &mut (Ipv4Addr, u16)) {
    ui.horizontal(|ui| {
        select_ipv4(ui, name, &mut selecting_address.0);
        ui.label(":");
        ui.add(DragValue::new(&mut selecting_address.1).speed(1));
    });
}
