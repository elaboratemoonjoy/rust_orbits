use std::{collections::btree_map::Range, ops::RangeInclusive};

use egui_macroquad::egui::{self, Response, Ui, Widget};
use macroquad::color::Color;

use crate::Body;

pub struct CelestialBody<'a> {
    body: &'a mut Body,
}
impl<'a> CelestialBody<'a> {
    pub fn from_body(body: &'a mut Body) -> Self {
        Self { body: body }
    }
}

impl<'a> Widget for CelestialBody<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        egui::Frame::NONE
            .fill(egui::Color32::DARK_GRAY)
            .inner_margin(5)
            .corner_radius(5)
            .show(ui, |ui| {
                ui.allocate_space(egui::vec2(ui.available_width(), 0.0));
                ui.horizontal(|ui| {
                    ui.add(
                        egui::TextEdit::singleline(&mut self.body.name)
                            .desired_width(0.0)
                            .clip_text(false)
                            .background_color(egui::Color32::DARK_GRAY),
                    );
                });

                ui.horizontal(|ui| {
                    ui.label("Mass");
                    ui.add(
                        egui::DragValue::new(&mut self.body.mass)
                            .update_while_editing(false)
                            .speed(1.0),
                    );
                    ui.label("Radius");
                    ui.add(
                        egui::DragValue::new(&mut self.body.radius)
                            .update_while_editing(false)
                            .speed(1.0),
                    );

                    let mut current_velocity = self.body.vel.length();
                    ui.label("Velocity");
                    ui.add(
                        egui::DragValue::new(&mut current_velocity)
                            .update_while_editing(false)
                            .speed(1.0)
                            .range(RangeInclusive::new(0, i32::MAX)),
                    );
                    if current_velocity != self.body.vel.length() {
                        self.body.vel = self.body.vel.normalize() * current_velocity;
                    }

                    let current_colour = self.body.colour;
                    let mut selected_rgb = [current_colour.r, current_colour.g, current_colour.b];
                    ui.label("Colour");
                    ui.color_edit_button_rgb(&mut selected_rgb);

                    self.body.colour.r = selected_rgb[0];
                    self.body.colour.g = selected_rgb[1];
                    self.body.colour.b = selected_rgb[2]
                })
            })
            .response
    }
}
