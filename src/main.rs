use macroquad::prelude::*;
use module::my_draw_line;

#[macroquad::main("My Macroquad Game")]
async fn main() {
    let mut points: Vec<(f32, f32)> = Vec::new();
    let mut draw: bool = true;
    let chaikin_turns: u8 = 7;
    let mut last = if points.len() >= 1 { Some(points[0]) } else { None };
    while !is_key_pressed(KeyCode::Escape) {
        clear_background(WHITE);
        if is_key_pressed(KeyCode::Enter) && draw {
            draw = false;
        }
        for p in &points {
            draw_rectangle(p.0, p.1, 2.0, 2.0, BLACK);
            match Some(last) {
                None => {
                    todo!();
                }
                _ => {
                    my_draw_line(last.unwrap(), *p);
                }
            }
        }
        if !draw && chaikin_turns != 0 {
            let new_points: Vec<(f32, f32)> = Vec::new();
            for p in &points {
            }
        }
        if is_mouse_button_down(MouseButton::Left) && draw {
            points.push(mouse_position());
            last = Some(mouse_position());
        }
        next_frame().await;
    }
}
