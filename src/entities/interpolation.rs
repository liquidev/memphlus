//! Components for smoothing out choppy movement.

use hecs::World;
use tetra::math::Vec2::{self};

use crate::interpolation::Interpolated;

use super::Position;

/// A component for interpolating the position of a component over many frames.
#[derive(Debug)]
pub struct InterpolatedPosition(pub Interpolated<Vec2<f32>>);

impl InterpolatedPosition {
   pub fn new(position: Vec2<f32>) -> Self {
      Self(Interpolated::new(position))
   }
}

/// Ticks the interpolation of positions.
pub fn tick_interpolation(world: &mut World) {
   for (_id, (ip, &Position(position))) in
      world.query_mut::<(&mut InterpolatedPosition, &Position)>()
   {
      ip.0.update(position);
   }
}
