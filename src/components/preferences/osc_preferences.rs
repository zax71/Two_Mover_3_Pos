use std::net::Ipv4Addr;

use egui::DragValue;

use crate::components::preferences::PreferenceItem;

#[derive(Debug, Clone, PartialEq)]
pub struct OscPreferences {
    host: (Ipv4Addr, u16),
    desk: (Ipv4Addr, u16),
}

impl OscPreferences {
    pub fn new() -> Self {
        Self {
            host: (Ipv4Addr::new(0, 0, 0, 0), 0),
            desk: (Ipv4Addr::new(192, 168, 0, 0), 8000),
        }
    }
}

impl PreferenceItem for OscPreferences {
    fn show(&mut self, ui: &mut egui::Ui, _global_state: &mut crate::app::GlobalState) {
        ui.vertical(|ui| {
            select_ip_port(ui, "Host", &mut self.host);
            select_ip_port(ui, "Desk", &mut self.desk);
        });
    }

    fn name(&self) -> &str {
        "🖧 OSC"
    }
}

/// TODO: Currently broken
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

fn select_ip_port(ui: &mut egui::Ui, name: &str, selecting_address: &mut (Ipv4Addr, u16)) {
    ui.horizontal(|ui| {
        select_ipv4(ui, name, &mut selecting_address.0);
        ui.label(":");
        ui.add(DragValue::new(&mut selecting_address.1).speed(1));
    });
}
