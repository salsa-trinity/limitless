use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_text(
            engine::limless_lib_logm(d.get_fps() as i32).as_str(),
            12,
            12,
            20,
            Color::BLACK,
        );
    }
}
