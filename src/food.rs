use crate::grid::Position;
use crate::snake::Snake;
use crate::grid::{grid_width, grid_height};
use rand::Rng;

pub struct Food {
    pub pos: Position,
}

impl Food {
    pub fn new_random(snake: &Snake) -> Self {
        let mut rng = rand::rng();

        let w = grid_width();
        let h = grid_height();

        // collect occupied positions
        let occupied: std::collections::HashSet<(i32,i32)> = snake
            .body
            .iter()
            .map(|p| (p.x, p.y))
            .collect();

        // Try until we find an empty cell (simple approach)
        loop {
            let x = rng.random_range(0..w);
            let y = rng.random_range(0..h);
            if !occupied.contains(&(x, y)) {
                return Food { pos: Position { x, y } };
            }
        }
    }
}
