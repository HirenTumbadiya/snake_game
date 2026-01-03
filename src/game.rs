use crate::grid::{CELL_SIZE, grid_height, grid_width};
use crate::snake::Snake;
use crate::food::Food;
use macroquad::prelude::*;

pub struct SnakeGame {
    snake: Snake,
    food: Food,
    game_over: bool,
    last_move_time: f64,
    move_interval: f64,
}

impl SnakeGame {
    pub fn new() -> Self {
        let snake = Snake::new();
        let food = Food::new_random(&snake);

        Self {
            snake,
            food,
            game_over: false,
            last_move_time: get_time(),
            move_interval: 0.15,
        }
    }

    pub fn update(&mut self) {
        // input (direction only) - ignore reverse inputs
        if is_key_pressed(KeyCode::Up) {
            let new_dir = crate::direction::Direction::Up;
            if !self.snake.dir.is_opposite(new_dir) {
                self.snake.dir = new_dir;
            }
        }
        if is_key_pressed(KeyCode::Down) {
            let new_dir = crate::direction::Direction::Down;
            if !self.snake.dir.is_opposite(new_dir) {
                self.snake.dir = new_dir;
            }
        }
        if is_key_pressed(KeyCode::Left) {
            let new_dir = crate::direction::Direction::Left;
            if !self.snake.dir.is_opposite(new_dir) {
                self.snake.dir = new_dir;
            }
        }
        if is_key_pressed(KeyCode::Right) {
            let new_dir = crate::direction::Direction::Right;
            if !self.snake.dir.is_opposite(new_dir) {
                self.snake.dir = new_dir;
            }
        }

        // if game over, listen for restart
        if self.game_over {
            if is_key_pressed(KeyCode::R) {
                let snake = Snake::new();
                self.food = Food::new_random(&snake);
                self.snake = snake;
                self.game_over = false;
                self.last_move_time = get_time();
            }
        } else {
            // movement (tick-based)
            let now = get_time();
            if now - self.last_move_time >= self.move_interval {
                // determine whether the snake will eat the food on this move
                let will_grow = self.snake.next_head() == self.food.pos;
                self.snake.move_forward(will_grow);
                if will_grow {
                    self.food = Food::new_random(&self.snake);
                }

                // check self-collision (head vs rest of body)
                let head = self.snake.head();
                if self.snake.body.iter().skip(1).any(|p| *p == head) {
                    self.game_over = true;
                }

                self.last_move_time = now;
            }
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

        // food
        let fpos = self.food.pos.to_screen();
        draw_rectangle(fpos.x, fpos.y, CELL_SIZE, CELL_SIZE, RED);

        // game over UI
        if self.game_over {
            let text = "Game Over - Press R to Restart";
            let font_size = 30.0;
            let tw = measure_text(text, None, font_size as u16, 1.0).width;
            let x = (grid_width() as f32 * CELL_SIZE - tw) / 2.0;
            let y = (grid_height() as f32 * CELL_SIZE) / 2.0;
            draw_text(text, x, y, font_size, RED);
        }
    }
}
