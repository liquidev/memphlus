//! Static colliders.

use ggez::graphics::Rect;
use hecs::{Entity, World};
use rapier2d::prelude::{ColliderBuilder, RigidBodyBuilder};

use crate::common::{mint, RectPoints};
use crate::physics::Physics;

use super::physics::{Collider, RigidBody};

/// A rectangular collider.
pub struct RectCollider;

impl RectCollider {
   /// Spawns a new rectangular collider into the world.
   pub fn spawn(world: &mut World, physics: &mut Physics, rect: Rect) -> Entity {
      let body = RigidBodyBuilder::new_static().translation(mint(rect.center_point())).build();
      let body = physics.rigid_bodies.insert(body);
      let collider = ColliderBuilder::cuboid(rect.w / 2.0, rect.h / 2.0).build();
      let collider =
         physics.colliders.insert_with_parent(collider, body, &mut physics.rigid_bodies);

      world.spawn((RectCollider, RigidBody(body), Collider(collider)))
   }
}
