mod body_config;
mod body_ui;
mod models;
mod sat_config;
mod setup;
mod sim;
mod vector_utils;
mod widgets;

use setup::setup_simulation;

use macroquad::prelude::*;

use crate::{models::SimConfig, sim::run_sim};

fn window_conf() -> Conf {
    Conf {
        window_title: "Orbits".to_owned(),
        window_width: 800,
        window_height: 800,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let config: SimConfig = setup_simulation().await;
    run_sim(config).await;
}
