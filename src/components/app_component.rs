use eframe::egui::Ui;

// Traits are kinda like abstract classes in Java

/// Used by any widget/component
pub trait AppComponent {
    // Make Context generic, due to how
    type Context;

    #[allow(unused)]
    fn add(ctx: &mut Self::Context, ui: &mut Ui) {}
}
