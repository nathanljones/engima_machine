mod common;
mod engima;
mod plugboard;
mod reflector;
mod rotor;
mod ui;
use macroquad::prelude::*;
#[macroquad::main(window_conf)]
async fn main() {
    loop {
        crate::ui::draw_window();
        next_frame().await
    }
}
fn window_conf() -> Conf {
    Conf {
        window_title: "Enigma Machine".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}
