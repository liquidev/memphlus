//! Entities and components.

use hecs::World;
use tetra::math::Vec2;
use tetra::{graphics, Context};

use crate::input::Input;
use crate::physics::Physics;
use crate::resources::Resources;
use crate::transform::TransformStack;

use self::camera::Camera;
use self::dead::Kill;
use self::interpolation::tick_interpolation;
use self::physics::tick_physics;
use self::player::Player;
use self::text::Text;
use self::zones::Zones;

pub mod camera;
pub mod colliders;
pub mod dead;
pub mod interpolation;
pub mod physics;
pub mod player;
pub mod text;
pub mod zones;

/// The position component.
pub struct Position(pub Vec2<f32>);

/// The default position is `(0.0, 0.0)`. This is useful when the position is to be updated by the
/// physics system.
impl Default for Position {
   fn default() -> Self {
      Self(Default::default())
   }
}

/// The rotation component, expressed in radians.
pub struct Rotation(pub f32);

/// The size component.
pub struct Size(pub Vec2<f32>);

/// Ticks all the systems.
pub fn tick_systems(ctx: &mut Context, world: &mut World, physics: &mut Physics, input: &Input) {
   Player::tick_controls(ctx, world, physics, input);
   Player::tick(world, physics);
   Kill::tick(world);
   tick_physics(world, physics);
   tick_interpolation(world);
   Camera::tick(world, physics);
}

/// Draws with all the systems.
pub fn draw_systems(
   ctx: &mut Context,
   tstack: &mut TransformStack,
   resources: &mut Resources,
   world: &mut World,
   physics: &mut Physics,
) -> anyhow::Result<()> {
   graphics::set_color_mask(ctx, false, false, true, true);
   Zones::draw(ctx, resources, world);

   graphics::set_color_mask(ctx, true, true, false, true);
   Text::draw(ctx, tstack, resources, world)?;
   Player::draw(ctx, world, physics)?;

   graphics::set_color_mask(ctx, true, true, true, true);
   Ok(())
}
