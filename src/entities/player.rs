//! Components and systems for the player entity.

use ggez::graphics::{self, DrawMode, DrawParam, MeshBuilder};
use ggez::Context;
use glam::Vec2;
use hecs::{Entity, World};
use rapier2d::prelude::{ColliderBuilder, RigidBodyBuilder};

use crate::assets::RemappableColors;
use crate::common::{mint, rect, vector, Transform};
use crate::physics::Physics;

use super::physics::{Collider, RigidBody};
use super::{Position, Size};

/// Marker component for marking that an entity can be controlled using platformer controls.
pub struct Platformer;

/// Marker component and namespace for player-related functions.
pub struct Player;

impl Player {
   /// Ticks the player controls.
   pub fn tick_controls(world: &mut World, physics: &mut Physics) {}

   /// Draws players.
   pub fn draw(ctx: &mut Context, world: &mut World, transform: Transform) -> anyhow::Result<()> {
      for (_id, (_, &Position(position), &Size(size))) in
         world.query_mut::<(&Player, &Position, &Size)>()
      {
         let rect = rect(position, size);
         let mesh = MeshBuilder::new()
            .rectangle(DrawMode::stroke(0.1), rect, RemappableColors::FOREGROUND)?
            .build(ctx)?;
         graphics::draw(ctx, &mesh, DrawParam::new().transform(transform))?;
      }
      Ok(())
   }

   /// Spawns a new player into the world.
   pub fn spawn(world: &mut World, physics: &mut Physics, position: Vec2) -> Entity {
      let size = vector(0.8, 0.8);

      let body =
         RigidBodyBuilder::new_dynamic().translation(mint(position)).lock_rotations().build();
      let body = physics.rigid_bodies.insert(body);
      let collider = ColliderBuilder::cuboid(size.x, size.y).build();
      let collider =
         physics.colliders.insert_with_parent(collider, body, &mut physics.rigid_bodies);

      world.spawn((
         Player,
         Position(position),
         Size(size),
         RigidBody(body),
         Collider(collider),
         Platformer,
      ))
   }
}
