use crate::draw::draw_block;
use piston_window::{types::Color, Context, G2d};
use rand::{thread_rng, Rng};
use std::collections::LinkedList;

const SNAKE_COLOR: Color = [0.00, 0.10, 0.01, 1.00];

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

impl TryFrom<i32> for Direction {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Direction::Up),
            1 => Ok(Direction::Down),
            2 => Ok(Direction::Left),
            3 => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Block {
    pub x: u32,
    pub y: u32,
}

impl Block {
    pub fn next_direction(&self, dir: Direction) -> Block {
        match dir {
            Direction::Up => Block {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Down => Block {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Block {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Block {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,
}

impl Snake {
    pub fn new(x: u32, y: u32) -> Snake {
        let dir: Direction = thread_rng().gen_range(0..=3).try_into().unwrap();
        let mut body = LinkedList::new();
        let (mut dx, mut dy) = (1, 1);
        match dir {
            Direction::Up => dy = 0,
            Direction::Down => dy = 2,
            Direction::Left => dx = 0,
            Direction::Right => dx = 2,
        };
        body.push_back(Block {
            x: x + 2 * dx - 2,
            y: y + 2 * dy - 2,
        });
        body.push_back(Block {
            x: x + dx - 1,
            y: y + dy - 1,
        });
        body.push_back(Block { x, y });

        Snake {
            direction: dir.try_into().unwrap(),
            body,
            tail: None,
        }
    }

    pub fn draw(&self, ctx: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, &ctx, g);
        }
    }

    pub fn head_position(&self) -> Result<Block, String> {
        let head_block = self.body.front().unwrap();
        Ok(Block {
            x: head_block.x,
            y: head_block.y,
        })
    }

    pub fn move_forward(&mut self, dir: Option<Direction>) {
        match dir {
            Some(d) => self.direction = d,
            None => (),
        };

        self.body.push_front(Block::next_direction(
            &self.head_position().unwrap(),
            self.direction,
        ));
        self.tail = Some(self.body.pop_back().unwrap());
    }

    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    pub fn next_head(&self, dir: Option<Direction>) -> Block {
        let curr_head = self.head_position().unwrap();

        let mut moving_dir = self.direction;
        match dir {
            Some(d) => moving_dir = d,
            None => (),
        };

        // Block::next_direction(self.head_position().as_ref().unwrap(), moving_dir)
        curr_head.next_direction(moving_dir)
    }

    pub fn restore_tail(&mut self) {
        let tail_block = self.tail.unwrap();
        self.body.push_back(tail_block);
    }

    pub fn overlap_tail(&self, pos: &Block) -> bool {
        let mut ch = 0;
        for block in &self.body {
            if pos == block {
                return true;
            }

            ch += 1;
            if ch == self.body.len() - 1 {
                break;
            }
        }

        return false;
    }
}
