use raylib::prelude::*;

pub struct App {
    m_rayLibHandle: RaylibHandle,
    m_thread: RaylibThread,
}

impl App {
    pub fn new() -> App {
        let (mut raylibHandle, thread) = raylib::init()
            .size(640, 480)
            .title("Particle Simulator")
            .build();

        App {
            m_rayLibHandle: raylibHandle,
            m_thread: thread,
        }
    }

    pub fn run(&mut self) {
        while !self.m_rayLibHandle.window_should_close() {
            self.update();
            self.draw();
        }
    }

    fn update(&mut self) {

    }

    fn draw(&mut self) {
        let mut d = self.m_rayLibHandle.begin_drawing(&self.m_thread);
        d.clear_background(Color::RAYWHITE);

        d.clear_background(Color::WHITE);
        d.draw_text("Particle Simulator", 100, 100, 50, Color::BLACK);
        d.draw_text("Start", 280, 250, 35, Color::BLACK);
        d.draw_text("Exit", 295, 320, 35, Color::BLACK);
    }
}
