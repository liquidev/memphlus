//! Static colliders.

use hecs::{Entity, World};
use rapier2d::prelude::{ColliderBuilder, InteractionGroups, RigidBodyBuilder};

use crate::common::{Rect, ToNalgebraVector2};
use crate::physics::{CollisionGroups, Physics};

use super::physics::{Collider, RigidBody};

/// A rectangular collider.
pub struct RectCollider;

impl RectCollider {
   /// Spawns a new rectangular collider into the world.
   pub fn spawn(world: &mut World, physics: &mut Physics, entity: Entity, rect: Rect) {
      let body = RigidBodyBuilder::new_static().translation(rect.center().nalgebra()).build();
      let body = physics.rigid_bodies.insert(body);
      let collider = ColliderBuilder::cuboid(rect.width / 2.0, rect.height / 2.0)
         .collision_groups(InteractionGroups::new(
            CollisionGroups::SOLIDS,
            CollisionGroups::ALL,
         ))
         .build();
      let collider =
         physics.colliders.insert_with_parent(collider, body, &mut physics.rigid_bodies);

      world.spawn_at(entity, (RectCollider, RigidBody(body), Collider(collider)));
   }
}
