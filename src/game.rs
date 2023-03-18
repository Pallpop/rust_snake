use piston_window::{types::Color, Context, G2d, Key};
use rand::{thread_rng, Rng};

use crate::{
    draw::{draw_block, draw_rectangle},
    snake::{Block, Direction, Snake},
};

const FOOD_COLOR: Color = [0.8, 0.0, 0.0, 1.0];
const BORDER_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
const GAMEOVER_COLOR: Color = [0.8, 0.0, 0.0, 0.5];

const MOVEING_PERIOD: f64 = 0.5;
const RESTART_TIME: f64 = 1.0;

pub struct Game {
    snake: Snake,

    food_exists: bool,
    food_block: Block,

    width: u32,
    height: u32,

    game_over: bool,
    waiting_time: f64,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Game {
        let mut rng = thread_rng();

        Game {
            snake: Snake::new(rng.gen_range(4..width - 4), rng.gen_range(4..height - 4)),
            food_exists: true,
            food_block: Block {
                x: rng.gen_range(1..width - 1),
                y: rng.gen_range(1..height - 1),
            },
            width,
            height,
            game_over: false,
            waiting_time: 0.0,
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }

        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => None,
        };

        if let Some(d) = dir {
            if d == self.snake.head_direction().opposite() {
                return;
            }
        } else {
            return;
        }

        self.update_snake(dir);
    }

    pub fn draw(&self, ctx: &Context, g: &mut G2d) {
        self.snake.draw(ctx, g);

        if self.food_exists {
            draw_block(FOOD_COLOR, self.food_block.x, self.food_block.y, ctx, g);
        }

        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, ctx, g);
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, ctx, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, ctx, g);
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, ctx, g);

        if self.game_over {
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, ctx, g);
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }

            return;
        }

        if !self.food_exists {
            self.add_food();
        }

        if self.waiting_time > MOVEING_PERIOD {
            self.update_snake(None);
        }
    }

    fn check_eating(&mut self) {
        let head_block = self.snake.head_position().unwrap();

        if self.food_exists && self.food_block == head_block {
            self.food_exists = false;
            self.snake.restore_tail();
        }
    }

    fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool {
        let next_block = self.snake.next_head(dir);

        if self.snake.overlap_tail(&next_block) {
            return false;
        }

        return next_block.x > 0
            && next_block.y > 0
            && next_block.x < self.width - 1
            && next_block.y < self.height - 1;
    }

    fn add_food(&mut self) {
        let mut rng = thread_rng();
        let mut x = rng.gen_range(1..self.width - 1);
        let mut y = rng.gen_range(1..self.height - 1);

        while self.snake.overlap_tail(&Block { x, y }) {
            x = rng.gen_range(1..self.width - 1);
            y = rng.gen_range(1..self.height - 1);
        }

        self.food_block = Block { x, y };
        self.food_exists = true;
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_if_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            self.game_over = true;
        }

        self.waiting_time = 0.0;
    }

    fn restart(&mut self) {
        let mut rng = thread_rng();

        self.snake = Snake::new(
            rng.gen_range(4..self.width - 4),
            rng.gen_range(4..self.height - 4),
        );
        self.food_block = Block {
            x: rng.gen_range(1..self.width - 1),
            y: rng.gen_range(1..self.height - 1),
        };
        self.food_exists = true;
        self.waiting_time = 0.0;
        self.game_over = false;
    }
}
