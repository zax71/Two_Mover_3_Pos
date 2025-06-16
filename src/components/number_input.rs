use eframe::egui::Widget;
use egui::{Response, Rgba, Ui};
use regex::Regex;

pub struct NumberInput<'a> {
    number: &'a mut Option<i32>,
}

impl NumberInput<'_> {
    fn is_int(s: &str) -> bool {
        let re = Regex::new(r"/^\d+\.?\d*$/").expect("Invalid Regex for int check");
        re.is_match(s)
    }
}

impl<'a> Widget for NumberInput<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut text: String = String::default();

        let input = ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut text);
            if Self::is_int(&text) {
                ui.colored_label(Rgba::RED, "Not a number");
            }
        });

        match Self::is_int(&text) {
            true => *self.number = Some(text.parse::<i32>().unwrap_or(0)),
            false => *self.number = None,
        }

        input.response
    }
}
