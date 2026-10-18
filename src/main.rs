use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Chess Game".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(BLACK);
        next_frame().await;
    }
}
