use macroquad::prelude::*;
use module::{ my_draw_line, chaikin_turns };

#[macroquad::main("My Macroquad Game")]
async fn main() {
    let mut points: Vec<(f32, f32)> = Vec::new();
    let mut draw: bool = true;
    while !is_key_pressed(KeyCode::Escape) {
        clear_background(WHITE);
        if is_key_pressed(KeyCode::Enter) && draw {
            draw = false;
            points = chaikin_turns(points.clone());
        }
        let mut last: Option<(f32, f32)> = None;
        for p in &points {
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
        if is_mouse_button_down(MouseButton::Left) && draw {
            points.push(mouse_position());
        }
        next_frame().await;
    }
}
