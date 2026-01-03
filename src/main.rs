use macroquad::prelude::*;


fn window_conf() -> Conf {
    Conf {
        window_title: "Snake Game".to_owned(),
        window_width: 800,  // Set your desired width
        window_height: 600, // Set your desired height
        window_resizable: false, // Disable resizing
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
