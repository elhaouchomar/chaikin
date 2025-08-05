use minifb::{ Icon, Window, WindowOptions, MouseButton, Key };
use image::GenericImageView;

fn main() {
    let (width, height) = (1000, 800);
    let img = image::from("media/logo.ico").unwrap();
    let mut win = Window::new("", height, width, WindowOptions::default()).unwrap();
    let mut buffer: Vec<u32> = vec![0; width * height];
    while win.is_open() && !win.is_key_down(Key::Escape) {
        if win.get_mouse_down(MouseButton::Left) {
            if let Some((x, y)) = win.get_mouse_pos(minifb::MouseMode::Clamp) {
                buffer[((y as usize) + 1) * height + (x as usize)] = 0x0000ff;
            }
        }
        win.update_with_buffer(&buffer, width, height).unwrap();
    }
}
