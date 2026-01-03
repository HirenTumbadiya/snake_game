mod direction;
mod food;
mod game;
mod grid;
mod snake;

use game::SnakeGame;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Snake Game".to_owned(),
        window_width: 800,       // Set your desired width
        window_height: 600,      // Set your desired height
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
