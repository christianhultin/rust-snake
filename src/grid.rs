use crate::constants::*;
use crate::direction::*;
use ggez::graphics::{self, Color};
use ggez::{self, Context, GameResult};
use oorandom::Rand32;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GridPosition {
    pub x: i16,
    pub y: i16,
}

impl From<(i16, i16)> for GridPosition {
    fn from(pos: (i16, i16)) -> Self {
        Self { x: pos.0, y: pos.1 }
    }
}

impl From<GridPosition> for graphics::Rect {
    fn from(pos: GridPosition) -> Self {
        graphics::Rect::new_i32(
            pos.x as i32 * GRID_CELL_SIZE.0 as i32,
            pos.y as i32 * GRID_CELL_SIZE.1 as i32,
            GRID_CELL_SIZE.0 as i32,
            GRID_CELL_SIZE.1 as i32,
        )
    }
}

impl GridPosition {
    pub fn new(x: i16, y: i16) -> Self {
        GridPosition { x, y }
    }

    pub fn random(rng: &mut Rand32, max_x: i16, max_y: i16) -> Self {
        (
            rng.rand_range(0..(max_x as u32)) as i16,
            rng.rand_range(0..(max_y as u32)) as i16,
        )
            .into()
    }

    pub fn new_from_move(pos: GridPosition, dir: Direction) -> Self {
        match dir {
            Direction::Up => (pos.x, pos.y - 1).into(),
            Direction::Right => (pos.x + 1, pos.y).into(),
            Direction::Down => (pos.x, pos.y + 1).into(),
            Direction::Left => (pos.x - 1, pos.y).into(),
        }
    }

    pub fn is_wall(&self) -> bool {
        if self.x > GRID_SIZE.0 || self.x < 0 {
            return true;
        }
        if self.y > GRID_SIZE.1 || self.y < 0 {
            return true;
        }
        false
    }

    pub fn draw(&self, ctx: &mut Context, color: Color) -> GameResult<()> {
        let rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            self.clone().into(),
            color,
        )?;
        graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
        Ok(())
    }
}
