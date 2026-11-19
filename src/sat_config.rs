use egui_macroquad::egui::{self, emath::Numeric};
use macroquad::prelude::*;

use crate::models::Body;
use crate::vector_utils;
use vector_utils::draw_vector_arrow;

const SPACEBLUE: Color = Color::new(0.00, 0.00, 0.05, 1.00);

fn draw_xy_axis() {
    draw_line(
        0.0,
        screen_height() / 2.0,
        screen_width(),
        screen_height() / 2.0,
        1.0,
        GREEN,
    );
    draw_line(
        screen_width() / 2.0,
        0.0,
        screen_width() / 2.0,
        screen_height(),
        1.0,
        RED,
    );
}

pub async fn configure_sattelite(planet: Body) -> Body {
    let offset = Vec2 {
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
    };
    let radius: f64 = 100.0;
    let ratio = planet.radius / radius;

    let mut name: String = "New Satellite".to_owned();
    let mut sat_radius: f64 = 0.0;
    let mut sat_mass: f64 = 0.0;
    let mut current_colour: Color = PURPLE;
    let mut angle: f32 = 0.0;
    let mut altitude: f64 = radius.clone();
    let mut speed: f64 = 7800.0;

    let mut save = false;

    loop {
        clear_background(SPACEBLUE);

        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("egui ❤ macroquad")
                .default_width(300.0)
                .show(egui_ctx, |ui| {
                    ui.spacing_mut().slider_width = 300.0;

                    ui.label(format!("Planet scale ratio: 1/{ratio}"));

                    ui.label("Name");
                    ui.add(egui::TextEdit::singleline(&mut name));

                    // Make a better input field widget for these kinda values..
                    ui.label("Size (radius)");
                    ui.add(egui::DragValue::new(&mut sat_radius).range(0.0..=f64::MAX));

                    ui.label("Mass (kg))");
                    ui.add(egui::DragValue::new(&mut sat_mass).range(0.0..=f64::MAX));

                    let mut selected_rgb = [current_colour.r, current_colour.g, current_colour.b];
                    ui.label("Colour");
                    ui.color_edit_button_rgb(&mut selected_rgb);
                    current_colour.r = selected_rgb[0];
                    current_colour.g = selected_rgb[1];
                    current_colour.b = selected_rgb[2];

                    ui.label("Angle");
                    ui.add(egui::Slider::new(&mut angle, 0.0..=359.999));

                    ui.label("Altitude");
                    ui.add(egui::Slider::new(&mut altitude, 0.0..=400.0));

                    ui.label("Speed");
                    ui.add(egui::Slider::new(&mut speed, 0.0..=15000.0));

                    ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                        let start_style = egui::Button::new(
                            egui::RichText::new("Save")
                                .size(25.0)
                                .color(egui::Color32::WHITE),
                        )
                        .fill(egui::Color32::from_rgb(0, 100, 0));

                        let start_button = ui.add_sized((200.0, 50.0), start_style);

                        if start_button.clicked() {
                            save = true;
                        }
                    });
                });
        });

        draw_xy_axis();

        draw_circle_lines(offset.x, offset.y, radius as f32, 2.0, WHITE);

        draw_arc(
            0.0 + offset.x,
            0.0 + offset.y,
            100,
            50.0,
            -angle,
            1.0,
            angle,
            BLUE,
        );

        let sat_pos = DVec2 {
            x: (radius + altitude) * angle.to_f64().to_radians().cos(),
            y: (radius + altitude) * -angle.to_f64().to_radians().sin(),
        };

        draw_line(
            offset.x,
            offset.y,
            offset.x + sat_pos.x as f32,
            offset.y + sat_pos.y as f32,
            1.0,
            WHITE,
        );

        let sat_rot = -sat_pos.normalize().perp() * speed / 100.0;

        draw_vector_arrow(offset, sat_pos.as_vec2(), sat_rot.as_vec2(), 2.0, BLUE);

        draw_circle(
            offset.x + sat_pos.x as f32,
            offset.y + sat_pos.y as f32,
            7.0,
            current_colour,
        );

        if save {
            return Body::new(
                "test".to_owned(),
                (sat_pos + planet.pos).into(),
                ((sat_rot.normalize() * speed) + planet.vel).into(),
                sat_radius,
                PURPLE,
                sat_mass,
            );
        }

        egui_macroquad::draw();
        next_frame().await;
    }
}
