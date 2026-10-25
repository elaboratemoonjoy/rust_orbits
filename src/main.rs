use macroquad::prelude::*;

const G: f64 = 6.67430e-11;

fn window_conf() -> Conf {
    Conf {
        window_title: "Snake Game".to_owned(),
        window_width: 800,
        window_height: 800,
        window_resizable: false,
        ..Default::default()
    }
}

struct Star {
    x: f32,
    y: f32,
    colour: Color,
}

#[derive(PartialEq)]
struct Body {
    pos: DVec2,
    vel: DVec2,
    radius: f64,
    colour: Color,
    mass: f64,
    grav_param: f64,
}
impl Body {
    fn new(pos: DVec2, vel: DVec2, radius: f64, colour: Color, mass: f64) -> Self {
        Self {
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

#[macroquad::main(window_conf)]
async fn main() {
    let offset_x = screen_width() / 2.0;
    let offset_y = screen_height() / 2.0;
    let frame_budget = 1.0 / 60.0;
    let meters_per_pixel = 1e5;

    let physics_hz = 12000000.0;
    let mut actual_hz = physics_hz;
    let physics_interval = 1.0 / physics_hz;
    let mut max_phys_count = physics_hz;

    let timewarp = 100000.0;
    let dt: f64 = physics_interval * timewarp;
    let mut elapsed_time: f64 = 0.0;
    let mut sim_time: f64 = 0.0;

    let stars: Vec<Star> = generate_background_stars();

    let mut bodies: Vec<Body> = vec![];

    bodies.push(Body::new(
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
        DVec2 { x: 0.0, y: 0.0 },
        DVec2 { x: 0.0, y: 0.0 },
        6.371e6,
        BLUE,
        5.972e24,
    ));

    loop {
        clear_background(SPACEBLUE);
        draw_background_stars(&stars);

        let physics_start = std::time::Instant::now();

        elapsed_time += get_frame_time() as f64;
        let mut steps = 0.0;
        while elapsed_time >= physics_interval && steps <= max_phys_count {
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
            steps += 1.0;
        }

        // To limit physics steps if simulation can't keep up.
        // Avoids fps death spiral
        let physics_elapsed = physics_start.elapsed().as_secs_f64();
        if steps > 0.0 && physics_elapsed > 0.0 {
            let seconds_per_step = physics_elapsed / steps as f64;
            max_phys_count = frame_budget / seconds_per_step;
            actual_hz = 1.0 / seconds_per_step
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
            &format!("Time passed: {:.0} seconds", sim_time.round() / 100.0),
            20.0,
            50.0,
            15.0,
            WHITE,
        );

        draw_text(
            &format!("Expected Hz: {:.0}", physics_hz),
            20.0,
            75.0,
            15.0,
            WHITE,
        );

        draw_text(
            &format!("Actual Hz: {:.0}", actual_hz),
            20.0,
            100.0,
            15.0,
            WHITE,
        );

        draw_fps();

        next_frame().await;
    }
}
