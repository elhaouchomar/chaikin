use macroquad::prelude::*;
use module::{ my_draw_line, chaikin_turns };


#[macroquad::main("My Macroquad Game")]
async fn main() {
    let delay = 2.0;
    let mut last_time = 0.0;
    // let mut counter = 0;
    let mut current_iter = 0;
    let mut points: Vec<(f32, f32)> = Vec::new();
    let mut smoothed_points: Vec<(f32, f32)> = Vec::new();
    let mut draw: bool = true;
    while !is_key_pressed(KeyCode::Escape) {
        clear_background(WHITE);
        if is_key_pressed(KeyCode::Enter) && draw {
            draw = false;
            smoothed_points = points.clone();
        }
        if draw == false {
            let now = get_time();
            if  now - last_time  > delay {
                if current_iter < 7 {
                    smoothed_points = chaikin_turns(&smoothed_points);
                    current_iter += 1;            
                } else {
                    current_iter = 0;
                    smoothed_points = points.clone();
                }
            }
        }
        

        if is_mouse_button_down(MouseButton::Left) && draw && (points.len() == 0 || points[points.len()- 1] != mouse_position()) {
            points.push(mouse_position());
            println!("new points >>> {:?}", points);
        }
        
        let mut last: Option<(f32, f32)> = None;
        for p in &smoothed_points {
            draw_rectangle(p.0, p.1, 2.0, 2.0, BLACK);
            match Some(last) {
                None => {
                    todo!();
                }
                _ => {
                    if draw {
                        my_draw_line(last.unwrap_or((0.0, 0.0)), *p);
                    }
                }
            }
            last = Some(*p);
        }
        next_frame().await;
    }
}
