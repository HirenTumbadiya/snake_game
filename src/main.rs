mod game;
mod snake;
mod grid;
mod direction;
mod food;

use macroquad::prelude::*;
use game::SnakeGame;


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
    let mut game = SnakeGame::new();

    loop {
        game.update();
        game.draw();

        next_frame().await;
    }
}