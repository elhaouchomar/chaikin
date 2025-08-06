use macroquad::prelude::*;

pub fn my_draw_line(point1: (f32, f32), point2: (f32, f32)) {
    if same_point(point1, point2) {
        return;
    }
    draw_line(point1.0, point1.1, point2.0, point2.1, 2.0, BLACK);
}

pub fn chaikin_turns(original_points: Vec<(f32, f32)>) {
    let mut points: Vec<(f32, f32)> = original_points.clone();
    for index in 1..=7 {
        let mut new_points: Vec<(f32, f32)> = points[..index].to_vec();
        new_points.append(&mut points[points.len() - (index as usize)..points.len()].to_vec());
        let mut first: Option<(f32, f32)> = None;
        for p in &points[index..points.len() - index].to_vec() {
            if first == None {
                first = Some(*p);
                continue;
            } else {
                let first_p = first.unwrap();
                println!("{:?}", first_p);
                println!("{:?}", p);
                let p1_x = first_p.0 * 0.75 + p.0 * 0.25;
                let p2_x = first_p.0 * 0.25 + p.0 * 0.75;
                let p1_y = first_p.1 * 0.75 + p.1 * 0.25;
                let p2_y = first_p.1 * 0.25 + p.1 * 0.75;
                new_points.push((p1_x, p1_y));
                new_points.push((p2_x, p2_y));
                my_draw_line((p1_x, p1_y), (p2_x, p2_y));
            }
        }
    }
}

fn same_point(point1: (f32, f32), point2: (f32, f32)) -> bool {
    if (point1.0 == point2.0 && point1.1 == point2.1) || (point1.0 == 0.0 && point1.1 == 0.0) {
        return true;
    } else {
        false
    }
}
