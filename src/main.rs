use raylib::color::Color;
use raylib::drawing::RaylibDraw;

fn main() {
    println!("Hello, world!");
    let window_width: i32 = 500;
    let window_height: i32 = 500;
    let (mut rl, thread) = raylib::init()
        .size(window_width, window_height)
        .title("Game")
        .build();
    rl.set_target_fps(60);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        d.draw_rectangle(5, 5, 100, 100, Color::RED);
        d.draw_fps(30, 30);
    }
}
