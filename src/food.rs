use ggez::{Context, GameResult};
use oorandom::Rand32;

use crate::{constants::GRID_SIZE, grid::GridPosition};

pub struct Food {
    pub pos: GridPosition,
    rng: Rand32,
}

impl Food {
    pub fn new(mut rng: Rand32) -> Self {
        Self {
            pos: GridPosition::random(&mut rng, GRID_SIZE.0, GRID_SIZE.1),
            rng,
        }
    }

    pub fn new_position(&mut self) {
        self.pos = GridPosition::random(&mut self.rng, GRID_SIZE.0, GRID_SIZE.1)
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        self.pos.draw(ctx, [1.0, 0.0, 0.0, 1.0].into())
    }
}
