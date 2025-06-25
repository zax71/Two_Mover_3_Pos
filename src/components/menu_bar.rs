use crate::app::GlobalState;

pub struct MenuBar {}

impl MenuBar {
    pub fn ui(ctx: &egui::Context, app_state: &mut GlobalState) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.menu_button("Theme", |ui| {
                    egui::widgets::global_theme_preference_buttons(ui);
                });

                ui.menu_button("Add", |ui| {
                    if ui.button("Light").clicked() {
                        app_state.add_light_window_status = true;
                        ui.close_menu();
                    }

                    if ui.button("Path").clicked() {
                        todo!("Implement adding paths");
                        //ui.close_menu();
                    }
                });
            });
        });
    }
}
