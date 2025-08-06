use macroquad::prelude::*;

pub fn my_draw_line(point1: (f32, f32), point2: (f32, f32)) {
    if point1.0 == point2.0 && point1.1 == point2.1 {
        return;
    }
    draw_line(point1.0, point1.1, point2.0, point2.1, 2.0, BLACK);
}
