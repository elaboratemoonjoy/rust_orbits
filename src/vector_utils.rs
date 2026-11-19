use macroquad::prelude::*;

pub fn rotate_vector_deg(vector: Vec2, angle_deg: f32) -> Vec2 {
    let angle = angle_deg.to_radians();
    let sin = angle.sin();
    let cos = angle.cos();

    let rotated = vec2(
        vector.x * cos - vector.y * sin,
        vector.x * sin + vector.y * cos,
    );

    rotated
}

pub fn rotate_vector_rad(vector: Vec2, angle_rad: f32) -> Vec2 {
    let sin = angle_rad.sin();
    let cos = angle_rad.cos();

    let rotated = vec2(
        vector.x * cos - vector.y * sin,
        vector.x * sin + vector.y * cos,
    );

    rotated
}

pub fn draw_vector_arrow(offset: Vec2, start: Vec2, end: Vec2, thickness: f32, color: Color) {
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

    let right_head = rotate_vector_deg(end, 135.0).normalize() * 20.0;
    let left_head = rotate_vector_deg(end, -135.0).normalize() * 20.0;
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
