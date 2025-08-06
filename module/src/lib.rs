use macroquad::prelude::*;

pub fn my_draw_line(point1: (f32, f32), point2: (f32, f32)) {
    draw_line(point1.0, point1.1, point2.0, point2.1, 2.0, BLACK);
}

pub fn chaikin_turns(original_points: &Vec<(f32, f32)>) -> Vec<(f32, f32)> {
    println!("old >>> {:?}", original_points);
    let new_p   =chaikin_smooth(&original_points);
    println!("new >>> {:?}", new_p);
    new_p
}

fn chaikin_smooth(points: &Vec<(f32, f32)>) -> Vec<(f32, f32)> {
    if points.len() < 2 {
        return points.to_vec();
    }

    let mut new_points: Vec<(f32, f32)> = Vec::new();
    new_points.push(points[0]);

    for i in 0..points.len() - 1 {
        let p1 = &points[i];
        let p2 = &points[i + 1];

        let np1 = 
        (     0.75 * p1.0 + 0.25 * p2.0,
             0.75 * p1.1 + 0.25 * p2.1,
        );
        let np2 = (
         0.25 * p1.0 + 0.75 * p2.0,
             0.25 * p1.1 + 0.75 * p2.1);

        new_points.push(np1);
        new_points.push(np2);
    }

    new_points.push(points[points.len() - 1]);

    new_points
}
