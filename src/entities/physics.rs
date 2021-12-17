//! Components and systems for handling physics objects.

use hecs::World;
use rapier2d::prelude::{ColliderHandle, RigidBodyHandle};

use crate::common::vector;
use crate::physics::Physics;

use super::Position;

/// The rigid body component.
pub struct RigidBody(pub RigidBodyHandle);

/// The collider component.
pub struct Collider(pub ColliderHandle);

/// Ticks physics objects, such that their Position component matches the actual position of
/// the body.
pub fn tick_physics(world: &mut World, physics: &mut Physics) {
   for (_id, (Position(position), &RigidBody(body_handle))) in
      world.query_mut::<(&mut Position, &RigidBody)>()
   {
      let body = &physics.rigid_bodies[body_handle];
      let translation = body.translation();
      *position = vector(translation.x, translation.y);
   }
}
