use std::ops::RangeInclusive;

use egui_macroquad::egui::{self};
use macroquad::prelude::*;

fn rotate_vector(vector: Vec2, angle: f32) -> Vec2 {
    let angle = angle.to_radians();
    let sin = angle.sin();
    let cos = angle.cos();

    let rotated = vec2(
        vector.x * cos - vector.y * sin,
        vector.x * sin + vector.y * cos,
    );

    rotated
}

fn draw_vector_arrow(offset: Vec2, start: Vec2, end: Vec2, thickness: f32, color: Color) {
    let line_start = offset + start;
    let line_end = line_start + end;
    draw_line(
        line_start.x,
        line_start.y,
        line_end.x,
        line_end.y,
        thickness,
        color,
    );

    let right_head = rotate_vector(end, 135.0).normalize() * 20.0;
    let left_head = rotate_vector(end, -135.0).normalize() * 20.0;
    draw_line(
        line_end.x,
        line_end.y,
        line_end.x + right_head.x,
        line_end.y + right_head.y,
        thickness,
        color,
    );
    draw_line(
        line_end.x,
        line_end.y,
        line_end.x + left_head.x,
        line_end.y + left_head.y,
        thickness,
        color,
    );
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Orbits".to_owned(),
        window_width: 1200,
        window_height: 1200,
        window_resizable: false,
        ..Default::default()
    }
}
const SPACEBLUE: Color = Color::new(0.00, 0.00, 0.05, 1.00);

#[macroquad::main(window_conf)]
async fn main() {
    let offset = Vec2 {
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
    };

    let mut angle: f32 = 0.0;
    let radius = 100.0;
    let mut altitude = radius.clone();
    let mut speed = 7800.0;

    loop {
        clear_background(SPACEBLUE);
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("egui ❤ macroquad")
                .default_width(300.0)
                .show(egui_ctx, |ui| {
                    ui.spacing_mut().slider_width = 300.0;
                    ui.label("Angle");
                    ui.add(egui::Slider::new(&mut angle, 0.0..=359.999));

                    ui.label("Altitude");
                    ui.add(egui::Slider::new(&mut altitude, 0.0..=400.0));

                    ui.label("Speed");
                    ui.add(egui::Slider::new(&mut speed, 0.0..=15000.0));
                });
        });
        draw_poly_lines(offset.x, offset.y, 50, radius, 0.0, 2.0, WHITE);
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

        let pointer = Vec2 {
            x: (radius + altitude) * angle.to_radians().cos(),
            y: (radius + altitude) * -angle.to_radians().sin(),
        };

        draw_line(
            offset.x,
            offset.y,
            offset.x + pointer.x,
            offset.y + pointer.y,
            1.0,
            WHITE,
        );

        let pointer_rot = -pointer.normalize().perp() * speed / 100.0;

        draw_line(
            offset.x + pointer.x,
            offset.y + pointer.y,
            offset.x + pointer_rot.x + pointer.x,
            offset.y + pointer_rot.y + pointer.y,
            1.0,
            WHITE,
        );

        draw_vector_arrow(offset, pointer, pointer_rot, 2.0, BLUE);

        draw_circle(pointer.x + offset.x, offset.y + pointer.y, 7.0, PURPLE);

        egui_macroquad::draw();
        next_frame().await;
    }
}
