//! Static colliders.

use hecs::{Entity, World};
use rapier2d::prelude::{ColliderBuilder, InteractionGroups, RigidBodyBuilder};

use crate::common::{Rect, ToNalgebraVector2};
use crate::physics::{CollisionGroups, Physics};

use super::physics::Collider;

/// A rectangular collider.
pub struct RectCollider;

impl RectCollider {
   /// Spawns a new rectangular collider into the world.
   pub fn spawn(world: &mut World, physics: &mut Physics, entity: Entity, rect: Rect) {
      let collider = ColliderBuilder::cuboid(rect.width / 2.0, rect.height / 2.0)
         .translation(rect.center().nalgebra())
         .collision_groups(InteractionGroups::new(
            CollisionGroups::SOLIDS,
            CollisionGroups::ALL,
         ))
         .build();
      let collider = physics.colliders.insert(collider);

      world.spawn_at(entity, (RectCollider, Collider(collider)));
   }
}
