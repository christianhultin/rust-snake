mod constants;
mod direction;
mod grid;
mod snaky_stuff;

use constants::{GRID_SIZE, MILLIS_PER_UPDATE, SCREEN_SIZE};
use direction::Direction;
use ggez::{
    event::{self, KeyCode, KeyMods},
    graphics, Context, GameResult,
};
use grid::GridPosition;
use oorandom::Rand32;
use snaky_stuff::food::Food;
use snaky_stuff::snake::Snake;
use std::time::{Duration, Instant};

struct GameState {
    snake: Snake,
    gameover: bool,
    food: Food,
    last_update: Instant,
    score: u16,
}

impl GameState {
    pub fn new() -> Self {
        let mut seed: [u8; 8] = [0; 8];
        getrandom::getrandom(&mut seed[..]).expect("Could not create RNG seed");
        let rng = Rand32::new(u64::from_ne_bytes(seed));

        GameState {
            snake: Snake::new(GridPosition::new(GRID_SIZE.0 / 2, GRID_SIZE.1 / 2)),
            gameover: false,
            food: Food::new(rng),
            last_update: Instant::now(),
            score: 0,
        }
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if !(Instant::now() - self.last_update >= Duration::from_millis(MILLIS_PER_UPDATE)) {
            return Ok(());
        }
        if !self.gameover {
            if self.snake.is_dead() {
                self.gameover = true;
                println!("Game over! Score: {}", self.score);
                return Ok(());
            }
            if self.snake.head == self.food.pos {
                self.score = self.score + 1;
                self.food.new_position();
                self.snake.update(true);
            } else {
                self.snake.update(false);
            }
            self.last_update = Instant::now();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());
        self.snake.draw(ctx)?;
        self.food.draw(ctx)?;
        graphics::present(ctx)?;
        ggez::timer::yield_now();
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        if let Some(dir) = Direction::from_keycode(keycode) {
            if dir != self.snake.direction && dir.inverse() != self.snake.direction {
                self.snake.direction = dir;
            }
        }
    }
}

fn main() {
    let (ref mut ctx, ref mut event_loop) = &mut ggez::ContextBuilder::new("snake", "ChrizTheWiz")
        .window_setup(ggez::conf::WindowSetup::default().title("Snake MF!"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()
        .unwrap();

    let state = &mut GameState::new();
    event::run(ctx, event_loop, state).unwrap()
}
