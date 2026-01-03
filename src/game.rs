use crate::grid::{CELL_SIZE, grid_height, grid_width};
use crate::snake::Snake;
use macroquad::prelude::*;

pub struct SnakeGame {
    snake: Snake,
    last_move_time: f64,
    move_interval: f64,
}

impl SnakeGame {
    pub fn new() -> Self {
        Self {
            snake: Snake::new(),
            last_move_time: get_time(),
            move_interval: 0.15,
        }
    }

    pub fn update(&mut self) {
        // input (direction only)
        if is_key_pressed(KeyCode::Up) {
            self.snake.dir = crate::direction::Direction::Up;
        }
        if is_key_pressed(KeyCode::Down) {
            self.snake.dir = crate::direction::Direction::Down;
        }
        if is_key_pressed(KeyCode::Left) {
            self.snake.dir = crate::direction::Direction::Left;
        }
        if is_key_pressed(KeyCode::Right) {
            self.snake.dir = crate::direction::Direction::Right;
        }

        // movement (tick-based)
        let now = get_time();
        if now - self.last_move_time >= self.move_interval {
            self.snake.move_forward();
            self.last_move_time = now;
        }
    }

    pub fn draw(&self) {
        clear_background(BLACK);

        // grid
        for x in 0..grid_width() {
            for y in 0..grid_height() {
                draw_rectangle_lines(
                    x as f32 * CELL_SIZE,
                    y as f32 * CELL_SIZE,
                    CELL_SIZE,
                    CELL_SIZE,
                    1.0,
                    DARKGRAY,
                );
            }
        }

        // snake
        for segment in &self.snake.body {
            let pos = segment.to_screen();
            draw_rectangle(pos.x, pos.y, CELL_SIZE, CELL_SIZE, GREEN);
        }
    }
}
