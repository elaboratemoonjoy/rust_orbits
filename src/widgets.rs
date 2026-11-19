use egui_macroquad::egui::{self};

pub struct ActionButton<'a> {
    text: egui::RichText,
    fill: egui::Color32,
    size: egui::Vec2,
    corner_radius: egui::CornerRadius,
    on_click: Box<dyn FnMut() + 'a>,
}

pub fn action_button<'a>(
    text: egui::RichText,
    fill: egui::Color32,
    size: egui::Vec2,
    corner_radius: egui::CornerRadius,
    on_click: impl FnMut() + 'a,
) -> ActionButton<'a> {
    ActionButton {
        text: text,
        fill: fill,
        size: size,
        corner_radius: corner_radius,
        on_click: Box::new(on_click),
    }
}

impl<'a> egui::Widget for ActionButton<'a> {
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        let button: egui::Button<'_> = egui::Button::new(self.text)
            .fill(self.fill)
            .corner_radius(self.corner_radius);

        let response = ui.add_sized(self.size, button);

        if response.clicked() {
            (self.on_click)();
        }

        response
    }
}
