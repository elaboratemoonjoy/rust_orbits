use egui_macroquad::egui::{self};
use macroquad::prelude::*;

use crate::models::Body;
use crate::widgets::action_button;
use crate::{SimConfig, body_ui::CelestialBody};

const SPACEBLUE: Color = Color::new(0.00, 0.00, 0.05, 1.00);

fn default_bodies() -> Vec<Body> {
    let mut bodies: Vec<Body> = vec![];

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

    bodies
}

pub async fn setup_simulation() -> SimConfig {
    let mut sim_config: SimConfig = SimConfig {
        timewarp: 1.0,
        physics_hz: 60.0,
        bodies: default_bodies(),
    };

    let mut start = false;
    let mut config_open = false;

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
                    .selected_text(format!("{:?}", sim_config.timewarp))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut sim_config.timewarp, 1.0, "Real time");
                        ui.selectable_value(&mut sim_config.timewarp, 10.0, "10x Real time");
                        ui.selectable_value(&mut sim_config.timewarp, 100.0, "100x Real time");
                        ui.selectable_value(&mut sim_config.timewarp, 1000.0, "1000x Real time");
                        ui.selectable_value(&mut sim_config.timewarp, 10000.0, "10000x Real time");
                    });

                egui::ComboBox::from_label("Select physics resolution")
                    .selected_text(format!("{:?}", sim_config.physics_hz))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut sim_config.physics_hz, 60.0, "60 hz");
                        ui.selectable_value(&mut sim_config.physics_hz, 240.0, "240 hz");
                        ui.selectable_value(&mut sim_config.physics_hz, 960.0, "960 hz");
                        ui.selectable_value(&mut sim_config.physics_hz, 10000.0, "10000 hz");
                    });

                ui.with_layout(egui::Layout::top_down_justified(egui::Align::Min), |ui| {
                    for i in 0..sim_config.bodies.len() {
                        ui.add(CelestialBody::from_body(&mut sim_config.bodies[i]));
                    }

                    ui.add(action_button(
                        egui::RichText::new("Add New")
                            .size(25.0)
                            .color(egui::Color32::WHITE),
                        egui::Color32::from_rgb(0, 100, 0),
                        (200.0, 50.0).into(),
                        5.0.into(),
                        || {
                            let mut new_body = Body {
                                ..Default::default()
                            };
                            new_body.name = "Change me".to_owned();
                            sim_config.bodies.push(new_body);
                        },
                    ));
                });

                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.add(action_button(
                        egui::RichText::new("Start")
                            .size(50.0)
                            .color(egui::Color32::WHITE),
                        egui::Color32::from_rgb(0, 100, 0),
                        (200.0, 50.0).into(),
                        5.0.into(),
                        || {
                            start = true;
                        },
                    ));
                });
            });
        });
        if config_open {
            // configure_sattelite(bodies[0]).await;
            config_open = false;
        }
        egui_macroquad::draw();
        next_frame().await;
    }

    sim_config
}
