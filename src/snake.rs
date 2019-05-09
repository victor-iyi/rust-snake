use std::collections::LinkedList;

use piston_window::{Context, G2d};
use piston_window::types::Color;

use draw::draw_block;

/// Snake color. (green)
const SNAKE_COLOR : Color = [0.0, 0.80, 0.0, 1.0];

/// Direction of the snake.
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

struct Block {
  x: i32,
  y: i32,
}

pub struct Snake {
  /// Direction the snake is currently travelling in.
  direction: Direction,
  /// Body of the snake.
  body: LinkedList<Block>,
  /// Snake's tail (grows when apple is eaten).
  tail: Option<Block>,
}

impl Snake {
  pub fn new(x: i32, y: i32) -> Snake {
    let mut body: LinkedList<Block> = LinkedList::new();
    // Snake is 3 block long by default.
    body.push_back(Block{
      x: x + 2,
      y: y,
    });
    // 2nd block.
    body.push_back(Block{
      x: x + 1,
      y: y,
    });
    // First block.
    body.push_back(Block {
      x,
      y,
    });

    // Create a new snake obj.
    Snake {
      direction: Direction::Right,
      body: body,
      tail:None,
    }
  }
}
