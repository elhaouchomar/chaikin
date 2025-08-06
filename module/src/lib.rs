use macroquad::prelude::*;

pub fn my_draw_line(point1: (f32, f32), point2: (f32, f32)) {
    if same_point(point1, point2) {
        return;
    }
    draw_line(point1.0, point1.1, point2.0, point2.1, 2.0, BLACK);
}

pub fn chaikin_turns(original_points: Vec<(f32, f32)>) -> Vec<(f32, f32)> {
    let mut points: Vec<(f32, f32)> = original_points.clone();
    points = chaikin_smooth(points, 2);
    points
}

fn chaikin_smooth(points: Vec<(f32, f32)>, iterations: usize) -> Vec<(f32, f32)> {
    let mut result = points;

    for _ in 0..iterations {
        let mut new_points = Vec::with_capacity(result.len() * 2);
        for i in 0..result.len() - 1 {
            let (x0, y0) = result[i];
            let (x1, y1) = result[i + 1];

            let q = (0.75 * x0 + 0.25 * x1, 0.75 * y0 + 0.25 * y1);

            let r = (0.25 * x0 + 0.75 * x1, 0.25 * y0 + 0.75 * y1);

            new_points.push(q);
            new_points.push(r);
        }
        result = new_points;
    }

    result
}

fn same_point(point1: (f32, f32), point2: (f32, f32)) -> bool {
    if (point1.0 == point2.0 && point1.1 == point2.1) || (point1.0 == 0.0 && point1.1 == 0.0) {
        return true;
    } else {
        false
    }
}
