use std::sync::atomic::{AtomicU64, Ordering};

use macroquad::prelude::{Color, DVec2};

const G: f64 = 6.67430e-11;

static NEXT_BODY_ID: AtomicU64 = AtomicU64::new(0);

#[derive(PartialEq, Clone)]
pub struct Body {
    pub id: u64,
    pub name: String,
    pub pos: DVec2,
    pub vel: DVec2,
    pub radius: f64,
    pub colour: Color,
    pub mass: f64,
    pub grav_param: f64,
}

impl Body {
    pub fn new(
        name: String,
        pos: DVec2,
        vel: DVec2,
        radius: f64,
        colour: Color,
        mass: f64,
    ) -> Self {
        Self {
            id: NEXT_BODY_ID.fetch_add(1, Ordering::Relaxed),
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

impl Default for Body {
    fn default() -> Self {
        Self {
            id: NEXT_BODY_ID.fetch_add(1, Ordering::Relaxed),
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
