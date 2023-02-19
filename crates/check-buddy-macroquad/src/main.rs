use macroquad::prelude::*;

#[macroquad::main("Checkbuddy")]
async fn main() {
    loop {
        clear_background(LIGHTGRAY);
        next_frame().await
    }
}
