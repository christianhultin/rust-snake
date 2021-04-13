mod constants;
mod direction;
mod food;
mod grid;
mod snake;

use crate::snake::Snake;
use constants::{GRID_SIZE, MILLIS_PER_UPDATE, SCREEN_SIZE};
use direction::Direction;
use food::Food;
use ggez::{
    event::{self, KeyCode, KeyMods},
    graphics, Context, GameResult,
};
use oorandom::Rand32;
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
            snake: Snake::new((GRID_SIZE.0 / 2, GRID_SIZE.1 / 2).into()),
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
            self.snake.update();
            if self.snake.is_dead() {
                self.gameover = true;
            }
        }
        if self.snake.head == self.food.pos {
            self.score = self.score + 1;
            println!("Score: {}", self.score);
            self.food.new_position();
        }
        self.last_update = Instant::now();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 1.0, 0.0, 1.0].into());
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
