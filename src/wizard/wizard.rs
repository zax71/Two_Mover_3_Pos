use egui::{Context, Ui};

/// A way to enter data in to a UI, displays each page to the user in succession finally offering a submit button
#[derive(Default)]
pub struct Wizard {
    screens: Vec<impl Fn(&mut Ui) -> egui::Response>,
    page: usize,
    title: String
}


impl Wizard {
    pub fn new(title: &str) -> Wizard {
        Wizard::default()
    }
    pub fn add_screen(&mut self, add_contents: impl Fn(&mut Ui) -> egui::Response) -> Wizard {
        self.screens.add(add_contents);

        self
    }

    pub fn show(&self, ctx: &Context) {
        egui::Window::new(self.title)
            .show(ctx, |ui| {
                screens[page];
                if ui.button("Next").clicked() {
                    page = page + 1
                }
    })
    }
}
