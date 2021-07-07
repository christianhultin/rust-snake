use crate::direction::Direction;
use crate::grid::GridPosition;
use ggez::graphics::Color;
use ggez::{Context, GameResult};

pub struct Snake {
    pub head: GridPosition,
    body: Vec<GridPosition>,
    pub direction: Direction,
}

impl Snake {
    pub fn new(pos: GridPosition) -> Snake {
        let b1 = GridPosition::new_from_move(pos, Direction::Left);
        let b2 = GridPosition::new_from_move(b1, Direction::Left);
        let b3 = GridPosition::new_from_move(b2, Direction::Left);
        Snake {
            head: pos,
            body: vec![b1, b2, b3],
            direction: Direction::Right,
        }
    }

    pub fn is_dead(&self) -> bool {
        if self.head.is_wall() {
            return true;
        }
        for seg in self.body.iter() {
            if seg.clone() == self.head {
                return true;
            }
        }
        false
    }

    pub fn update(&mut self, should_grow: bool) {
        let new_head_position = GridPosition::new_from_move(self.head, self.direction);
        self.body.insert(0, self.head);
        self.head = new_head_position;
        if !should_grow {
            self.body.pop();
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let color: Color = [1.0, 1.0, 1.0, 1.0].into();
        for seg in self.body.iter() {
            seg.draw(ctx, color)?;
        }
        self.head.draw(ctx, color)?;
        Ok(())
    }
}
