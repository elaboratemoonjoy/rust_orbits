mod body_ui;

use egui_macroquad::egui::{self};
use macroquad::prelude::*;

use crate::body_ui::CelestialBody;

const G: f64 = 6.67430e-11;

fn window_conf() -> Conf {
    Conf {
        window_title: "Orbits".to_owned(),
        window_width: 800,
        window_height: 800,
        window_resizable: false,
        ..Default::default()
    }
}

#[derive(Default)]
struct SimConfig {
    timewarp: f64,
    physics_hz: f64,
    bodies: Vec<Body>,
}

struct Star {
    x: f32,
    y: f32,
    colour: Color,
}

#[derive(PartialEq, Clone)]
struct Body {
    name: String,
    pos: DVec2,
    vel: DVec2,
    radius: f64,
    colour: Color,
    mass: f64,
    grav_param: f64,
}

impl Default for Body {
    fn default() -> Self {
        Self {
            name: Default::default(),
            pos: Default::default(),
            vel: Default::default(),
            radius: Default::default(),
            colour: Default::default(),
            mass: Default::default(),
            grav_param: Default::default(),
        }
    }
}

impl Body {
    fn new(name: String, pos: DVec2, vel: DVec2, radius: f64, colour: Color, mass: f64) -> Self {
        Self {
            name,
            pos,
            vel,
            radius,
            colour,
            mass,
            grav_param: G * mass,
        }
    }
}

const SPACEBLUE: Color = Color::new(0.00, 0.00, 0.05, 1.00);

const STAR_COLOURS: [Color; 3] = [
    Color::from_rgba(255, 80, 40, 255),
    Color::from_rgba(100, 160, 255, 255),
    Color::from_rgba(255, 220, 150, 255),
];

fn generate_background_stars() -> Vec<Star> {
    let mut stars: Vec<Star> = Vec::new();
    for _ in 0..100 {
        let x = macroquad::rand::gen_range(0.0, screen_width());
        let y = macroquad::rand::gen_range(0.0, screen_height());

        let colour = STAR_COLOURS[macroquad::rand::gen_range(0, STAR_COLOURS.len())];

        stars.push(Star {
            x: x,
            y: y,
            colour: colour,
        });
    }

    stars
}

fn draw_background_stars(stars: &Vec<Star>) {
    for star in stars {
        draw_circle(star.x, star.y, 2.0, star.colour);
    }
}

async fn setup_simulation() -> SimConfig {
    let mut sim_config = SimConfig {
        ..Default::default()
    };
    let mut bodies: Vec<Body> = vec![];

    // Default
    bodies.push(Body::new(
        "Sattelite".to_owned(),
        DVec2 {
            x: 0.0,
            y: 6.371e6 + 200e3,
        },
        DVec2 { x: 8000.0, y: 0.0 },
        2.0,
        GREEN,
        1e4,
    ));
    bodies.push(Body::new(
        "Earth".to_owned(),
        DVec2 { x: 0.0, y: 0.0 },
        DVec2 { x: 0.0, y: 0.0 },
        6.371e6,
        BLUE,
        5.972e24,
    ));

    let mut timewarp = 1.0;
    let mut physics_hz = 60.0;
    let mut start = false;

    while !start {
        clear_background(SPACEBLUE);
        egui_macroquad::ui(|egui_ctx| {
            egui::CentralPanel::default().show(egui_ctx, |ui| {
                let mut visuals = egui::Visuals::dark();
                visuals.override_text_color = Some(egui::Color32::WHITE);

                egui_ctx.set_visuals(visuals);

                ui.vertical_centered(|ui| {
                    ui.label(
                        egui::RichText::new("Config")
                            .heading()
                            .color(egui::Color32::from_rgb(255, 255, 255)),
                    );
                });

                egui::ComboBox::from_label("Select time warp")
                    .selected_text(format!("{:?}", timewarp))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut timewarp, 1.0, "Real time");
                        ui.selectable_value(&mut timewarp, 10.0, "10x Real time");
                        ui.selectable_value(&mut timewarp, 100.0, "100x Real time");
                        ui.selectable_value(&mut timewarp, 1000.0, "1000x Real time");
                        ui.selectable_value(&mut timewarp, 10000.0, "10000x Real time");
                    });

                egui::ComboBox::from_label("Select physics resolution")
                    .selected_text(format!("{:?}", physics_hz))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut physics_hz, 60.0, "60 hz");
                        ui.selectable_value(&mut physics_hz, 240.0, "240 hz");
                        ui.selectable_value(&mut physics_hz, 960.0, "960 hz");
                        ui.selectable_value(&mut physics_hz, 10000.0, "10000 hz");
                    });

                ui.with_layout(egui::Layout::top_down_justified(egui::Align::Min), |ui| {
                    for i in 0..bodies.len() {
                        ui.add(CelestialBody::from_body(&mut bodies[i]));
                    }

                    let new_style = egui::Button::new(
                        egui::RichText::new("Add New")
                            .size(25.0)
                            .color(egui::Color32::WHITE),
                    )
                    .fill(egui::Color32::from_rgb(0, 100, 0));

                    let new_button = ui.add_sized((50.0, 50.0), new_style);
                    if new_button.clicked() {
                        let mut new_body = Body {
                            ..Default::default()
                        };
                        new_body.name = "Change me".to_owned();
                        bodies.push(new_body);
                    }
                });

                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    let start_style = egui::Button::new(
                        egui::RichText::new("Start")
                            .size(50.0)
                            .color(egui::Color32::WHITE),
                    )
                    .fill(egui::Color32::from_rgb(0, 100, 0));

                    let start_button = ui.add_sized((200.0, 50.0), start_style);

                    if start_button.clicked() {
                        sim_config.timewarp = timewarp;
                        sim_config.physics_hz = physics_hz;
                        sim_config.bodies = bodies.clone();

                        start = true;
                    }
                });
            });
        });
        egui_macroquad::draw();
        next_frame().await;
    }

    sim_config
}

#[macroquad::main(window_conf)]
async fn main() {
    let config: SimConfig = setup_simulation().await;

    let offset_x = screen_width() / 2.0;
    let offset_y = screen_height() / 2.0;
    let meters_per_pixel = 1e5;

    let physics_hz = config.physics_hz;
    let physics_interval = 1.0 / physics_hz;

    let timewarp = config.timewarp;
    let dt: f64 = physics_interval * timewarp;
    let mut elapsed_time: f64 = 0.0;
    let mut sim_time: f64 = 0.0;

    let stars: Vec<Star> = generate_background_stars();

    let mut bodies = config.bodies;

    loop {
        clear_background(SPACEBLUE);
        draw_background_stars(&stars);

        elapsed_time += get_frame_time() as f64;
        while elapsed_time >= physics_interval {
            sim_time += dt;
            for i in 0..bodies.len() {
                let body_pos = bodies[i].pos;
                let mut sum_acceleration = DVec2::ZERO;

                for (j, target) in bodies.iter().enumerate() {
                    if i == j {
                        continue;
                    }

                    let dir = target.pos - body_pos;

                    let r = dir.length();

                    // GM = grav_param
                    // gravity acceleration vector = GM * 1/r³ * dir * dt
                    if r != 0.0 {
                        sum_acceleration += target.grav_param * (1.0 / r.powi(3)) * dir * dt;
                    }
                }

                let body = &mut bodies[i];
                body.vel += sum_acceleration;
            }

            for body in &mut bodies {
                body.pos += body.vel * dt;
            }

            elapsed_time -= physics_interval;
        }

        for body in &bodies {
            let pixel_x = (body.pos.x / meters_per_pixel) as f32;
            let pixel_y = (body.pos.y / meters_per_pixel) as f32;
            let mut pixel_r = (body.radius / meters_per_pixel) as f32;
            if pixel_r < 1.0 {
                pixel_r = 1.0;
            }
            let colour = body.colour;
            draw_circle(pixel_x + offset_x, pixel_y + offset_y, pixel_r, colour);
        }

        draw_text(
            &format!("Time passed: {:.2} seconds", sim_time),
            20.0,
            50.0,
            20.0,
            WHITE,
        );

        draw_text(
            &format!("Physics Hz (real time based): {:.0}", physics_hz),
            20.0,
            75.0,
            20.0,
            WHITE,
        );

        draw_fps();
        next_frame().await;
    }
}
