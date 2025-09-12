use egui::DragValue;
use vector3d::Vector3d;

pub mod add_light_window;
pub mod add_path;
pub mod output_section;

/// Displays a component with three input boxes for x,y,z coordinates
pub fn select_vec(ui: &mut egui::Ui, name: &str, selecting_vec: &mut Vector3d<f64>) {
    ui.horizontal(|ui| {
        ui.label(name);
        ui.add(DragValue::new(&mut selecting_vec.x).speed(0.1));
        ui.add(DragValue::new(&mut selecting_vec.y).speed(0.1));
        ui.add(DragValue::new(&mut selecting_vec.z).speed(0.1));
    });
}
