//! Zones, the core mechanic of the game.

use hecs::{Component, Entity, World};
use tetra::graphics::{Color, DrawParams};
use tetra::Context;
use vek::Vec2;

use crate::common::{vector, WhiteTexture};
use crate::physics::Physics;
use crate::resources::Resources;

use super::{Position, Rotation, Size};

pub trait Zone: Component {
   /// Returns the unique, constant index of the zone.
   fn index() -> usize;
}

macro_rules! zone_index {
   ($zone:ty, $index:expr) => {
      impl Zone for $zone {
         fn index() -> usize {
            $index
         }
      }
   };
}

/// Marker component for platformer zones.
pub struct PlatformerZone;
zone_index!(PlatformerZone, 1);

/// Marker component for deadly zones.
pub struct DeadlyZone;
zone_index!(DeadlyZone, 2);

/// Namespace struct for zone-related systems.
pub struct Zones;

impl Zones {
   /// The maximum zone index.
   pub const MAX: usize = 32;

   /// Draws zones to the screen.
   pub fn draw(ctx: &mut Context, resources: &mut Resources, world: &mut World) {
      Self::draw_zone::<PlatformerZone>(ctx, resources, world);
      Self::draw_zone::<DeadlyZone>(ctx, resources, world);
   }

   /// Draws a specific type of zone to the screen.
   fn draw_zone<T>(ctx: &mut Context, resources: &mut Resources, world: &mut World)
   where
      T: Zone,
   {
      let WhiteTexture(white_texture) = resources.get().unwrap();

      for (_id, (_zone_tag, &Position(position), &Size(size), &Rotation(rotation))) in
         world.query_mut::<(&T, &Position, &Size, &Rotation)>()
      {
         let color = Color {
            r: 0.0,
            g: 0.0,
            b: T::index() as f32 / Self::MAX as f32,
            a: 1.0,
         };
         white_texture.draw(
            ctx,
            DrawParams::new()
               .origin(vector(0.5, 0.5))
               .scale(size)
               .position(position)
               .rotation(rotation)
               .color(color),
         );
      }
   }

   /// Spawns a zone into the world.
   pub fn spawn(
      world: &mut World,
      physics: &mut Physics,
      kind: impl Zone,
      center: Vec2<f32>,
      size: Vec2<f32>,
      rotation: f32,
   ) -> Entity {
      world.spawn((kind, Position(center), Size(size), Rotation(rotation)))
   }
}
