use std::ops::RangeInclusive;

use egui_macroquad::egui::{self};
use macroquad::prelude::*;

fn draw_point_on_circle(radius: f32, angle: f32) {
    let pos_x = radius * (angle as f32).to_radians().cos(); 
    let mut pos_y = (radius.powf(2.0) - pos_x.powf(2.0)).sqrt(); 

    if angle < 180.0{
        pos_y = pos_y * -1.0;
    }

    draw_line(offset_x, offset_y, offset_x + pos_x, offset_y + pos_y, 1.0, WHITE);

    draw_circle(pos_x + offset_x, offset_y + pos_y, 7.0, PURPLE);

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
    let offset_x = screen_width() / 2.0;
    let offset_y = screen_height() / 2.0;

    let mut angle = 45.0;
    let radius = 100.0;

    loop{
        clear_background(SPACEBLUE);
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("egui ❤ macroquad")
            .default_width(300.0)
            .show(egui_ctx, |ui| {
                ui.spacing_mut().slider_width = 300.0;
                ui.label("Angle");
                ui.add(egui::Slider::new(&mut angle, 0.0..=359.999));
            });
        });
        draw_poly_lines(offset_x, offset_y, 50, 100.0, 0.0, 2.0, WHITE);
        draw_line(0.0, screen_height()/2.0, screen_width(), screen_height()/2.0, 1.0, GREEN);
        draw_line(screen_width()/2.0, 0.0, screen_width()/2.0, screen_height(), 1.0, RED);

        draw_arc(0.0+offset_x, 0.0+offset_y, 100, 50.0, -angle, 1.0, angle, BLUE);


        egui_macroquad::draw();
        next_frame().await;
    }
}