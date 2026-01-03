use std::collections::VecDeque;
use crate::grid::Position;
use crate::direction::Direction;

pub struct Snake {
    pub body: VecDeque<Position>,
    pub dir: Direction,
}

impl Snake {
    pub fn new() -> Self {
        let mut body = VecDeque::new();
        body.push_back(Position { x: 5, y: 5 });

        Self {
            body,
            dir: Direction::Right,
        }
    }

    pub fn head(&self) -> Position {
        *self.body.front().unwrap()
    }

    pub fn next_head(&self) -> Position {
        let head = self.head();

        let new_head = match self.dir {
            Direction::Up => Position { x: head.x, y: head.y - 1 },
            Direction::Down => Position { x: head.x, y: head.y + 1 },
            Direction::Left => Position { x: head.x - 1, y: head.y },
            Direction::Right => Position { x: head.x + 1, y: head.y },
        };

        new_head.wrap()
    }

    pub fn move_forward(&mut self, grow: bool) {
        let next = self.next_head();
        self.body.push_front(next);
        if !grow {
            self.body.pop_back();
        }
    }
}
