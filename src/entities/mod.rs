//! Entities and components.

use hecs::World;
use tetra::math::Vec2;
use tetra::Context;

use crate::input::Input;
use crate::physics::Physics;

use self::interpolation::tick_interpolation;
use self::physics::tick_physics;
use self::player::Player;

pub mod colliders;
pub mod interpolation;
pub mod physics;
pub mod player;

/// The position component.
pub struct Position(pub Vec2<f32>);

/// The default position is `(0.0, 0.0)`. This is useful when the position is to be updated by the
/// physics system.
impl Default for Position {
   fn default() -> Self {
      Self(Default::default())
   }
}

/// The size component.
pub struct Size(pub Vec2<f32>);

/// Ticks all the systems.
pub fn tick_systems(ctx: &mut Context, world: &mut World, physics: &mut Physics, input: &Input) {
   Player::tick_controls(ctx, world, physics, input);
   tick_physics(world, physics);
   tick_interpolation(world);
}

/// Draws with all the systems.
pub fn draw_systems(ctx: &mut Context, world: &mut World) -> anyhow::Result<()> {
   Player::draw(ctx, world)?;
   Ok(())
}
