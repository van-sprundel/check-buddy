use check_buddy_gui::run;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Checkbuddy".to_owned(),
        window_height: 400,
        window_width: 400,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    run().await;
}
