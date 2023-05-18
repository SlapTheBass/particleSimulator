use raylib::prelude::*;

pub fn Run() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Particle Simulator")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_text("Particle Simulator", 100, 100, 50, Color::BLACK);
        d.draw_text("Start", 280, 250, 35, Color::BLACK);
        d.draw_text("Exit", 295, 320, 35, Color::BLACK);
    }
}