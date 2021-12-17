use serde::{Deserialize, Serialize};

/// Spatial representation of a ball or hole.
pub type Point = [f32; 2];

/// Golf field, set of balls and holes.
#[derive(Debug, Serialize, Deserialize)]
pub struct GolfField {
  pub holes: Vec<Point>,
  pub balls: Vec<Point>,
}
