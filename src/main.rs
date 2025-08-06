use macroquad::prelude::*;
use chaikin::{ChaikinState, draw_points, draw_lines, draw_control_point_circles};

#[macroquad::main("Chaikin's Algorithm Animation")]
async fn main() {
    let mut state = ChaikinState::new();
    let mut mouse_pressed_last_frame = false;

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if is_key_pressed(KeyCode::Enter) {
            state.start_animation();
        }

        if is_key_pressed(KeyCode::C) {
            state.clear();
        }

        let mouse_pos = mouse_position();
        let mouse_pressed = is_mouse_button_down(MouseButton::Left);
        let mouse_just_pressed = mouse_pressed && !mouse_pressed_last_frame;
        let mouse_just_released = !mouse_pressed && mouse_pressed_last_frame;

        if mouse_just_pressed {
            state.start_drag(mouse_pos.0, mouse_pos.1);

            if state.dragging.is_none() && !state.animation_running {
                state.add_point(mouse_pos.0, mouse_pos.1);
            }
        }

        if mouse_pressed && state.dragging.is_some() {
            state.update_drag(mouse_pos.0, mouse_pos.1);
        }

        if mouse_just_released {
            state.end_drag();
        }

        mouse_pressed_last_frame = mouse_pressed;
        state.update();

        clear_background(BLACK);

        if state.current_points.len() > 1 {
            draw_lines(&state.current_points, BLUE, 2.0);
        }

        if !state.control_points.is_empty() {
            draw_control_point_circles(&state.control_points);
        }

        if state.animation_running && state.current_points.len() > 2 {
            draw_points(&state.current_points, GREEN, 2.0);
        }

        next_frame().await;
    }
}
