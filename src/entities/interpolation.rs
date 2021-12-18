//! Components for smoothing out choppy movement.

use hecs::World;
use tetra::math::Vec2::{self};

use super::Position;

/// A component for interpolating the position of a component over many frames.
pub struct InterpolatedPosition {
   pub previous_position: Vec2<f32>,
   pub current_position: Vec2<f32>,
}

impl InterpolatedPosition {
   pub fn new(position: Vec2<f32>) -> Self {
      Self {
         previous_position: position,
         current_position: position,
      }
   }

   /// Linearly interpolates the position according to the given alpha.
   pub fn lerp(&self, alpha: f32) -> Vec2<f32> {
      let alpha = alpha.clamp(0.0, 1.0);
      Vec2::lerp(self.previous_position, self.current_position, alpha)
   }
}

/// Ticks the interpolation of positions.
pub fn tick_interpolation(world: &mut World) {
   for (_id, (ip, &Position(position))) in
      world.query_mut::<(&mut InterpolatedPosition, &Position)>()
   {
      ip.previous_position = ip.current_position;
      ip.current_position = position;
   }
}
