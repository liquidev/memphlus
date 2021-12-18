//! Components and systems for the player entity.

use hecs::{Entity, World};
use rapier2d::math::Isometry;
use rapier2d::prelude::{
   CoefficientCombineRule, ColliderBuilder, Cuboid, InteractionGroups, RigidBodyBuilder,
   RigidBodyHandle,
};
use tetra::graphics::mesh::{GeometryBuilder, ShapeStyle};
use tetra::graphics::DrawParams;
use tetra::math::Vec2;
use tetra::{time, Context};

use crate::assets::RemappableColors;
use crate::common::{rect, vector, ToNalgebraVector2};
use crate::input::{Button, Input};
use crate::physics::{CollisionGroups, Physics};

use super::interpolation::InterpolatedPosition;
use super::physics::{Collider, RigidBody};
use super::{Position, Size};

/// Component for storing state for platformer controls..
pub struct Platformer {
   remaining_jump_ticks: u8,
   jump_buffer: u8,
   air_time: u8,
}

impl Platformer {
   pub fn new() -> Self {
      Self {
         remaining_jump_ticks: 0,
         jump_buffer: 0,
         air_time: 0,
      }
   }
}

/// Marker component and namespace for player-related functions.
pub struct Player;

impl Player {
   const SIZE: [f32; 2] = [0.8, 0.8];

   /// Ticks the player controls.
   pub fn tick_controls(
      ctx: &mut Context,
      world: &mut World,
      physics: &mut Physics,
      input: &Input,
   ) {
      const SPEED: f32 = 175.0;
      const JUMP_STRENGTH: f32 = 700.0;
      // The number of ticks during which the jump button can be held down to adjust height.
      const JUMP_SUSTAIN: u8 = 10;
      // The number of ticks of leeway during which one can be falling down and still jump when
      // they land.
      const JUMP_LEEWAY: u8 = 8;
      // The number of ticks during which you can still jump after falling down a ledge.
      const COYOTE_TIME: u8 = 10;

      for (_id, (_, platformer, &RigidBody(body_handle))) in
         world.query_mut::<(&Player, &mut Platformer, &RigidBody)>()
      {
         {
            let body = &mut physics.rigid_bodies[body_handle];
            if input.button_down(ctx, Button::Left) {
               body.apply_force(vector(-SPEED, 0.0).nalgebra(), true);
            }
            if input.button_down(ctx, Button::Right) {
               body.apply_force(vector(SPEED, 0.0).nalgebra(), true);
            }
         }

         if input.button_just_pressed(ctx, Button::Jump) {
            platformer.jump_buffer = JUMP_LEEWAY;
         }
         if Self::is_on_ground(physics, body_handle) {
            platformer.air_time = COYOTE_TIME;
         }
         let body = &mut physics.rigid_bodies[body_handle];
         if platformer.jump_buffer > 0 && platformer.air_time > 0 {
            platformer.remaining_jump_ticks = JUMP_SUSTAIN;
            platformer.jump_buffer = 0;
            let velocity = *body.linvel();
            body.set_linvel(vector(velocity.x, 0.0).nalgebra(), true);
         }

         let body = &mut physics.rigid_bodies[body_handle];
         if input.button_down(ctx, Button::Jump) && platformer.remaining_jump_ticks > 0 {
            let strength = platformer.remaining_jump_ticks as f32 / JUMP_SUSTAIN as f32;
            let strength = strength.powf(6.0);
            body.apply_force(vector(0.0, -JUMP_STRENGTH * strength).nalgebra(), true);
         }
         if !input.button_down(ctx, Button::Jump) {
            platformer.remaining_jump_ticks = 0;
         }

         platformer.air_time = platformer.air_time.saturating_sub(1);
         platformer.jump_buffer = platformer.jump_buffer.saturating_sub(1);
         platformer.remaining_jump_ticks = platformer.remaining_jump_ticks.saturating_sub(1);

         let velocity = *body.linvel();
         let decelerated_x = velocity.x * 0.8;
         body.set_linvel(Vec2::new(decelerated_x, velocity.y).nalgebra(), true);
      }
   }

   /// Returns whether the (player's) physics body is standing on solid ground.
   fn is_on_ground(physics: &Physics, body: RigidBodyHandle) -> bool {
      let body = &physics.rigid_bodies[body];
      let size = Vec2::from_slice(&Self::SIZE) / 2.0;
      let shape = Cuboid::new(vector(size.x - 0.01, 0.01).nalgebra());
      let translation = body.translation();
      let translation = vector(translation.x, translation.y + size.y);
      physics
         .query
         .cast_shape(
            &physics.colliders,
            &Isometry::new(translation.nalgebra(), 0.0),
            &vector(0.0, 0.1).nalgebra(),
            &shape,
            1.0,
            InteractionGroups::new(CollisionGroups::PLAYER, CollisionGroups::SOLIDS),
            None,
         )
         .is_some()
   }

   /// Draws players.
   pub fn draw(ctx: &mut Context, world: &mut World) -> anyhow::Result<()> {
      for (_id, (_, position, &Size(size))) in
         world.query_mut::<(&Player, &InterpolatedPosition, &Size)>()
      {
         let position = position.lerp(time::get_blend_factor(ctx));
         let rect = rect(position - size / 2.0, size);
         let mesh = GeometryBuilder::new()
            .set_color(RemappableColors::BACKGROUND)
            .rectangle(ShapeStyle::Fill, rect)?
            .set_color(RemappableColors::FOREGROUND)
            .rectangle(ShapeStyle::Stroke(0.1), rect)?
            .build_mesh(ctx)?;
         mesh.draw(ctx, DrawParams::new());
      }
      Ok(())
   }

   /// Spawns a new player into the world.
   pub fn spawn(world: &mut World, physics: &mut Physics, position: Vec2<f32>) -> Entity {
      let size = Vec2::from_slice(&Self::SIZE);

      let body =
         RigidBodyBuilder::new_dynamic().translation(position.nalgebra()).lock_rotations().build();
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
