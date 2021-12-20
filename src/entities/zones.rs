//! Zones, the core mechanic of the game.

use hecs::{Component, Entity, World};
use rapier2d::prelude::{ColliderBuilder, InteractionGroups};
use tetra::graphics::{Color, DrawParams};
use tetra::Context;
use vek::Vec2;

use crate::common::{vector, ToNalgebraVector2, WhiteTexture};
use crate::physics::{CollisionGroups, Physics};
use crate::resources::Resources;

use super::physics::Collider;
use super::{Position, Rotation, Size};

/// Auto-generated helper trait for providing zones with palette indices.
pub trait ZoneIndex: Component {
   /// Returns the unique, constant index of the zone.
   fn index() -> usize;
}

macro_rules! zone_index {
   ($zone:ty, $index:expr) => {
      impl ZoneIndex for $zone {
         fn index() -> usize {
            $index
         }
      }
   };
}

/// Zone spawning behavior.
#[allow(unused_variables)]
pub trait ZoneSpawn: Component {
   /// Injects extra behavior for spawning the entity into the world.
   fn spawn(world: &mut World, physics: &mut Physics, entity: Entity) {}
}

/// Marker component for platformer zones.
pub struct PlatformerZone;
zone_index!(PlatformerZone, 1);

impl ZoneSpawn for PlatformerZone {}

/// Marker component for deadly zones.
pub struct DeadlyZone;
zone_index!(DeadlyZone, 2);

impl ZoneSpawn for DeadlyZone {
   fn spawn(world: &mut World, physics: &mut Physics, entity: Entity) {
      let Position(position) = *world.get(entity).unwrap();
      let Size(size) = *world.get(entity).unwrap();
      let Rotation(rotation) = *world.get(entity).unwrap();
      let collider = ColliderBuilder::cuboid(size.x / 2.0, size.y / 2.0)
         .translation(position.nalgebra())
         .rotation(rotation)
         .collision_groups(InteractionGroups::new(
            CollisionGroups::DEADLY,
            CollisionGroups::PLAYER,
         ))
         .build();
      let collider = physics.colliders.insert(collider);
      world.insert_one(entity, Collider(collider)).unwrap();
   }
}

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
      T: ZoneIndex,
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
   pub fn spawn<Z>(
      world: &mut World,
      physics: &mut Physics,
      kind: Z,
      center: Vec2<f32>,
      size: Vec2<f32>,
      rotation: f32,
   ) -> Entity
   where
      Z: ZoneIndex + ZoneSpawn,
   {
      let entity = world.spawn((kind, Position(center), Size(size), Rotation(rotation)));
      <Z as ZoneSpawn>::spawn(world, physics, entity);
      entity
   }
}
