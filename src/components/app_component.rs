use eframe::egui::Ui;

// Traits are kinda like abstract classes in Java

/// Used by any widget/component
pub trait AppComponent {
    // Make Context generic, due to how

    #[allow(unused)]
    fn add(&self, ctx: &mut egui::Context, ui: &mut Ui) {}
}
