use crate::direction::Direction;
use crate::grid::GridPosition;
use ggez::{graphics, Context, GameResult};
use std::collections::LinkedList;

pub struct Snake {
    head: GridPosition,
    body: LinkedList<GridPosition>,
    pub direction: Direction,
}

impl Snake {
    pub fn new(pos: GridPosition) -> Snake {
        Snake {
            head: pos,
            body: LinkedList::new(),
            direction: Direction::Right,
        }
    }

    pub fn update(&mut self) {
        let new_head_position = GridPosition::new_from_move(self.head, self.direction);
        self.body.push_front(self.head);
        self.body.pop_back();
        self.head = new_head_position;
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        for seg in self.body.iter() {
            Self::draw_position(&seg, ctx)?;
        }
        Self::draw_position(&self.head, ctx)?;
        Ok(())
    }

    fn draw_position(pos: &GridPosition, ctx: &mut Context) -> GameResult<()> {
        let rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            pos.clone().into(),
            [0.3, 0.3, 0.0, 1.0].into(),
        )?;
        graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
        Ok(())
    }
}
