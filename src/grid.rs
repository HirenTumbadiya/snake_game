use macroquad::prelude::Vec2;

pub const CELL_SIZE: f32 = 20.0;

pub fn grid_width() -> i32 {
    (800.0 / CELL_SIZE) as i32
}

pub fn grid_height() -> i32 {
    (600.0 / CELL_SIZE) as i32
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn to_screen(self) -> Vec2 {
        Vec2::new(
            self.x as f32 * CELL_SIZE,
            self.y as f32 * CELL_SIZE,
        )
    }

    pub fn wrap(self) -> Position {
        let w = grid_width();
        let h = grid_height();
        let mut x = self.x;
        let mut y = self.y;

        if x < 0 {
            x = w - 1;
        } else if x >= w {
            x = 0;
        }

        if y < 0 {
            y = h - 1;
        } else if y >= h {
            y = 0;
        }

        Position { x, y }
    }
}
