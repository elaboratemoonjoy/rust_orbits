use egui_macroquad::egui::{self};
use macroquad::prelude::*;

use crate::models::{Body, SetupConfig};
use crate::setup;
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
        DVec2 { x: 8000.0, y: 0.0 },
        6.371e6,
        BLUE,
        5.972e24,
    ));

    bodies
}

struct NewBodyPrompt {
    show: bool,
    edit_body_id: Option<u64>,
    edit_reference_body_id: Option<u64>,
}
impl Default for NewBodyPrompt {
    fn default() -> Self {
        Self {
            show: false,
            edit_body_id: None,
            edit_reference_body_id: None,
        }
    }
}

pub async fn setup_simulation() -> SimConfig {
    // TODO add reference body on the body widget
    // TODO add edit button on body widget
    // TODO when creating new body, automatically open the sat_config

    let mut setup_config: SetupConfig = SetupConfig {
        timewarp: 1000.0,
        physics_hz: 60.0,
        bodies: default_bodies(),
        ref_planet_map: vec![(0, 1)],
    };

    let mut new_body_promt = NewBodyPrompt {
        ..Default::default()
    };

    let mut start = false;
    while !start {
        clear_background(SPACEBLUE);
        egui_macroquad::ui(|egui_ctx| {
            egui::CentralPanel::default().show(egui_ctx, |ui| {
                let mut visuals = egui::Visuals::dark();
                visuals.override_text_color = Some(egui::Color32::WHITE);

                egui_ctx.set_visuals(visuals);

                if new_body_promt.show {
                    egui::Window::new("New body").show(egui_ctx, |ui| {
                        egui::ComboBox::from_label("Select reference planet")
                            .selected_text(format!(
                                "{}",
                                new_body_promt
                                    .edit_reference_body_id
                                    .map_or("None".to_string(), |id| id.to_string())
                            ))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut new_body_promt.edit_reference_body_id,
                                    None,
                                    "None".to_owned(),
                                );
                                for body in &setup_config.bodies {
                                    ui.selectable_value(
                                        &mut new_body_promt.edit_reference_body_id,
                                        body.id.clone().into(),
                                        body.id.clone().to_string(),
                                    );
                                }
                            });

                        if !new_body_promt.edit_reference_body_id.is_none() {
                            let selected_reference = setup_config
                                .bodies
                                .iter()
                                .find(|x| x.id == new_body_promt.edit_reference_body_id.unwrap())
                                .unwrap();

                            ui.label(selected_reference.name.to_string());
                        }

                        ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                            ui.horizontal(|ui| {
                                if ui.button("Create").clicked() {
                                    let new_body = Body {
                                        ..Default::default()
                                    };

                                    new_body_promt.edit_body_id = new_body.id.into();
                                    new_body_promt.show = false;
                                    setup_config.ref_planet_map.push((
                                        new_body.id,
                                        new_body_promt.edit_reference_body_id.unwrap(),
                                    ));
                                    setup_config.bodies.push(new_body);
                                }

                                if ui.button("Cancel").clicked() {
                                    new_body_promt = NewBodyPrompt {
                                        ..Default::default()
                                    };
                                }
                            });
                        });
                    });
                }

                ui.vertical_centered(|ui| {
                    ui.label(
                        egui::RichText::new("Config")
                            .heading()
                            .color(egui::Color32::from_rgb(255, 255, 255)),
                    );
                });

                egui::ComboBox::from_label("Select time warp")
                    .selected_text(format!("{:?}", setup_config.timewarp))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut setup_config.timewarp, 1.0, "Real time");
                        ui.selectable_value(&mut setup_config.timewarp, 10.0, "10x Real time");
                        ui.selectable_value(&mut setup_config.timewarp, 100.0, "100x Real time");
                        ui.selectable_value(&mut setup_config.timewarp, 1000.0, "1000x Real time");
                        ui.selectable_value(
                            &mut setup_config.timewarp,
                            10000.0,
                            "10000x Real time",
                        );
                    });

                egui::ComboBox::from_label("Select physics resolution")
                    .selected_text(format!("{:?}", setup_config.physics_hz))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut setup_config.physics_hz, 60.0, "60 hz");
                        ui.selectable_value(&mut setup_config.physics_hz, 240.0, "240 hz");
                        ui.selectable_value(&mut setup_config.physics_hz, 960.0, "960 hz");
                        ui.selectable_value(&mut setup_config.physics_hz, 10000.0, "10000 hz");
                    });

                ui.with_layout(egui::Layout::top_down_justified(egui::Align::Min), |ui| {
                    for i in 0..setup_config.bodies.len() {
                        ui.add(CelestialBody::from_body(&mut setup_config.bodies[i]));
                    }

                    ui.add(action_button(
                        egui::RichText::new("Add New")
                            .size(25.0)
                            .color(egui::Color32::WHITE),
                        egui::Color32::from_rgb(0, 100, 0),
                        (200.0, 50.0).into(),
                        5.0.into(),
                        || {
                            new_body_promt = NewBodyPrompt {
                                ..Default::default()
                            };
                            new_body_promt.show = true;
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

        // if edit_body {
        //     // Create new body prompt
        //     edit_body = false;
        // }
        egui_macroquad::draw();
        next_frame().await;
    }

    return SimConfig::from(setup_config);
}
