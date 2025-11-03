use egui::DragValue;
use vector3d::Vector3d;

pub mod add_light_window;
pub mod add_path;
pub mod debug_point_at;
pub mod output_section;
pub mod preferences;
pub mod select_lights_modal;
pub mod select_path_modal;
pub mod toggleable_item;

/// Displays a component with three input boxes for x,y,z coordinates
pub fn select_vec(ui: &mut egui::Ui, name: &str, selecting_vec: &mut Vector3d<f64>) {
    ui.horizontal(|ui| {
        ui.label(name);
        ui.label("x:");
        ui.add(DragValue::new(&mut selecting_vec.x).speed(0.1));
        ui.label("y:");
        ui.add(DragValue::new(&mut selecting_vec.y).speed(0.1));
        ui.label("z:");
        ui.add(DragValue::new(&mut selecting_vec.z).speed(0.1));
        // TODO: Show tooltip on click too
        ui.label("?").on_hover_text("These are cartesian coordinates with x being right, y being forwards and z up (like it is in mathematical contexts)");
    });
}
