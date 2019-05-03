use piston_window::{Rectangle, Context, G2d};
use piston_window::types::Color;

const BLOCK_SIZE: f64 = 25.0;

pub fn to_coord(game_coord: i32) -> f64 {
  (game_coord as f64) * BLOCK_SIZE
}