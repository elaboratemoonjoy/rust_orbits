use crate::models::{SimConfig, Star};
use macroquad::prelude::*;

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

pub async fn run_sim(config: SimConfig) {
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
