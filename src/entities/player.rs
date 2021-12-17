//! Components and systems for the player entity.

use std::time::{Duration, Instant};

use ggez::graphics::{self, DrawMode, DrawParam, MeshBuilder};
use ggez::Context;
use glam::Vec2;
use hecs::{Entity, World};
use rapier2d::math::Isometry;
use rapier2d::prelude::{
   CoefficientCombineRule, ColliderBuilder, Cuboid, InteractionGroups, RigidBodyBuilder,
   RigidBodyHandle,
};

use crate::assets::RemappableColors;
use crate::common::{mint, rect, vector, Transform};
use crate::input::{Button, Input};
use crate::physics::{CollisionGroups, Physics};

use super::interpolation::InterpolatedPosition;
use super::physics::{Collider, RigidBody};
use super::{Position, Size};

/// Component for storing state for platformer controls..
pub struct Platformer {
   remaining_jump_ticks: u32,
}

impl Platformer {
   pub fn new() -> Self {
      Self {
         remaining_jump_ticks: 0,
      }
   }
}

/// Marker component and namespace for player-related functions.
pub struct Player;

impl Player {
   const SIZE: [f32; 2] = [0.8, 0.8];

   /// Ticks the player controls.
   pub fn tick_controls(world: &mut World, physics: &mut Physics, input: &Input) {
      const SPEED: f32 = 175.0;
      const JUMP_STRENGTH: f32 = 700.0;
      const JUMP_SUSTAIN: u32 = 7;

      for (_id, (_, platformer, &RigidBody(body_handle))) in
         world.query_mut::<(&Player, &mut Platformer, &RigidBody)>()
      {
         {
            let body = &mut physics.rigid_bodies[body_handle];
            if input.button_down(Button::Left) {
               body.apply_force(mint(vector(-SPEED, 0.0)), true);
            }
            if input.button_down(Button::Right) {
               body.apply_force(mint(vector(SPEED, 0.0)), true);
            }
         }

         if input.button_just_pressed(Button::Jump) && Self::is_on_ground(physics, body_handle) {
            platformer.remaining_jump_ticks = JUMP_SUSTAIN;
         }
         let body = &mut physics.rigid_bodies[body_handle];
         if input.button_down(Button::Jump) && platformer.remaining_jump_ticks > 0 {
            let strength = platformer.remaining_jump_ticks as f32 / JUMP_SUSTAIN as f32;
            let strength = strength.powf(4.0);
            body.apply_force(mint(vector(0.0, -JUMP_STRENGTH * strength)), true);
         }
         platformer.remaining_jump_ticks = platformer.remaining_jump_ticks.saturating_sub(1);

         let velocity = *body.linvel();
         let decelerated_x = velocity.x * 0.8;
         body.set_linvel(mint(Vec2::new(decelerated_x, velocity.y)), true);
      }
   }

   /// Returns whether the (player's) physics body is standing on solid ground.
   fn is_on_ground(physics: &Physics, body: RigidBodyHandle) -> bool {
      let body = &physics.rigid_bodies[body];
      let size = Vec2::from_slice(&Self::SIZE) / 2.0;
      let shape = Cuboid::new(mint(vector(size.x - 0.01, 0.01)));
      let translation = body.translation();
      let translation = vector(translation.x, translation.y + size.y);
      physics
         .query
         .cast_shape(
            &physics.colliders,
            &Isometry::new(mint(translation), 0.0),
            &mint(vector(0.0, 0.1)),
            &shape,
            1.0,
            InteractionGroups::new(CollisionGroups::PLAYER, CollisionGroups::SOLIDS),
            None,
         )
         .is_some()
   }

   /// Draws players.
   pub fn draw(
      ctx: &mut Context,
      world: &mut World,
      transform: Transform,
      alpha: f32,
   ) -> anyhow::Result<()> {
      for (_id, (_, position, &Size(size))) in
         world.query_mut::<(&Player, &InterpolatedPosition, &Size)>()
      {
         let position = position.lerp(alpha);
         let rect = rect(position - size / 2.0, size);
         let mesh = MeshBuilder::new()
            .rectangle(DrawMode::fill(), rect, RemappableColors::BACKGROUND)?
            .rectangle(DrawMode::stroke(0.1), rect, RemappableColors::FOREGROUND)?
            .build(ctx)?;
         graphics::draw(ctx, &mesh, DrawParam::new().transform(transform))?;
      }
      Ok(())
   }

   /// Spawns a new player into the world.
   pub fn spawn(world: &mut World, physics: &mut Physics, position: Vec2) -> Entity {
      let size = Vec2::from_slice(&Self::SIZE);

      let body =
         RigidBodyBuilder::new_dynamic().translation(mint(position)).lock_rotations().build();
      let body = physics.rigid_bodies.insert(body);
      let collider = ColliderBuilder::cuboid(size.x / 2.0, size.y / 2.0)
         .friction(0.0)
         .friction_combine_rule(CoefficientCombineRule::Min)
         .collision_groups(InteractionGroups::new(
            CollisionGroups::PLAYER,
            CollisionGroups::SOLIDS,
         ))
         .build();
      let collider =
         physics.colliders.insert_with_parent(collider, body, &mut physics.rigid_bodies);

      world.spawn((
         Player,
         Position(position),
         InterpolatedPosition::new(position),
         Size(size),
         RigidBody(body),
         Collider(collider),
         Platformer::new(),
      ))
   }
}
