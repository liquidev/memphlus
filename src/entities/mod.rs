//! Entities and components.

use ggez::Context;
use glam::Vec2;
use hecs::World;

use crate::common::Transform;
use crate::physics::Physics;

use self::physics::tick_physics;
use self::player::Player;

mod physics;
pub mod player;

/// The position component.
pub struct Position(pub Vec2);

/// The default position is `(0.0, 0.0)`. This is useful when the position is to be updated by the
/// physics system.
impl Default for Position {
   fn default() -> Self {
      Self(Default::default())
   }
}

/// The size component.
pub struct Size(pub Vec2);

/// Ticks all the systems.
pub fn tick_systems(world: &mut World, physics: &mut Physics) {
   Player::tick_controls(world, physics);
   tick_physics(world, physics);
}

/// Draws with all the systems.
pub fn draw_systems(
   ctx: &mut Context,
   world: &mut World,
   transform: Transform,
) -> anyhow::Result<()> {
   Player::draw(ctx, world, transform)?;
   Ok(())
}
