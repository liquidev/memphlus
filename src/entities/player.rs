//! Components and systems for the player entity.

use std::time::Duration;

use hecs::{Entity, World};
use rapier2d::math::Isometry;
use rapier2d::prelude::{
   CoefficientCombineRule, ColliderBuilder, Cuboid, InteractionGroups, RigidBodyBuilder,
   RigidBodyHandle,
};
use tetra::graphics::mesh::{GeometryBuilder, ShapeStyle};
use tetra::graphics::DrawParams;
use tetra::math::Vec2;
use tetra::Context;

use crate::assets::RemappableColors;
use crate::common::{rect, vector, ToNalgebraVector2, ToVekVec2};
use crate::input::{Button, Input};
use crate::physics::{CollisionGroups, Physics};
use crate::tween::{easings, Tween};

use super::camera::Camera;
use super::dead::{Alive, Dead, Kill};
use super::interpolation::InterpolatedPosition;
use super::physics::{Collider, RigidBody};
use super::{Position, Size};

/// A player's morph state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Morph {
   /// No morph kind detected yet.
   None = 0,

   /// - Movement: can move freely in 8 directions. Velocity is not limited.
   /// - Zone: none
   /// - Player shape: metaball
   Unshaped = 1,

   /// - Movement: your typical platformer movement.
   /// - Zone: white
   /// - Player shape: cube
   Platformer = 2,
}

impl Morph {
   pub const FROM_U8: &'static [Morph] = &[Morph::None, Morph::Unshaped, Morph::Platformer];
}

/// Component for storing state of unshaped controls.
pub struct Unshaped {}

impl Unshaped {
   pub fn new() -> Self {
      Self {}
   }
}

/// Component for storing state of platformer controls.
pub struct Platformer {
   remaining_jump_ticks: u8,
   jump_buffer: u8,
   air_time: u8,

   /// Animation of the `width:height` aspect ratio, used for squishing and stretching.
   aspect_ratio: Tween<f32>,
   /// The previous velocity, for tracking when the player falls onto the ground.
   previous_velocity: Vec2<f32>,
}

impl Platformer {
   pub fn new() -> Self {
      Self {
         remaining_jump_ticks: 0,
         jump_buffer: 0,
         air_time: 0,
         aspect_ratio: Tween::new(1.0),
         previous_velocity: vector(0.0, 0.0),
      }
   }
}

/// Marker component and namespace for player-related functions.
pub struct Player {
   checkpoint: Vec2<f32>,
   /// Animation triggered right when the player spawns in, or turns into a different morph.
   spawn_animation: Tween<f32>,
}

impl Player {
   const SIZE: [f32; 2] = [0.8, 0.8];

   pub fn new(checkpoint: Vec2<f32>) -> Self {
      let mut player = Self {
         checkpoint,
         spawn_animation: Tween::new(1.0),
      };
      player.start_spawn_animation();
      player
   }

   /// Starts the spawning animation.
   pub fn start_spawn_animation(&mut self) {
      self.spawn_animation.start(0.0, 1.0, Duration::from_millis(250), easings::bounce_out);
   }

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
         world.query_mut::<Alive<(&Player, &mut Platformer, &RigidBody)>>()
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
            if input.button_just_pressed(ctx, Button::Jump) {
               platformer.aspect_ratio.start(
                  0.6,
                  1.0,
                  Duration::from_millis(350),
                  easings::cubic_out,
               );
            }
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

         let velocity = body.linvel();
         if platformer.previous_velocity.y > 0.01 && velocity.y <= 0.01 {
            platformer.aspect_ratio.start(1.5, 1.0, Duration::from_millis(250), easings::cubic_out);
         }
         platformer.previous_velocity = velocity.vek();
      }
   }

   /// Returns whether the (player's) physics body is standing on solid ground.
   fn is_on_ground(physics: &Physics, body: RigidBodyHandle) -> bool {
      let body = &physics.rigid_bodies[body];
      let size = Vec2::from_slice(&Self::SIZE) / 2.0;
      let shape = Cuboid::new(vector(size.x - 0.05, 0.01).nalgebra());
      let translation = body.translation();
      let translation = vector(translation.x, translation.y + size.y);
      physics
         .query
         .intersection_with_shape(
            &physics.colliders,
            &Isometry::new(translation.nalgebra(), 0.0),
            &shape,
            InteractionGroups::new(CollisionGroups::PLAYER, CollisionGroups::SOLIDS),
            None,
         )
         .is_some()
   }

   /// Ticks players.
   pub fn tick(world: &mut World, physics: &mut Physics) {
      // Kill the player if they touch a deadly collision group.
      let mut kill = Vec::new();
      for (id, (_, &Size(size), &RigidBody(body_handle))) in
         world.query_mut::<Alive<(&Player, &Size, &RigidBody)>>()
      {
         let body = &physics.rigid_bodies[body_handle];
         if let Some(_) = physics.query.intersection_with_shape(
            &physics.colliders,
            body.position(),
            &Cuboid::new((size / 2.0).nalgebra()),
            InteractionGroups::new(CollisionGroups::PLAYER, CollisionGroups::DEADLY),
            None,
         ) {
            kill.push(id);
         }
      }
      for player in kill {
         let _ = world.insert_one(player, Kill::after(1));
      }

      // Respawn all dead players.
      let mut respawn = Vec::new();
      for (id, (player, &RigidBody(body_handle), &Dead)) in
         world.query_mut::<(&Player, &RigidBody, &Dead)>()
      {
         let body = &mut physics.rigid_bodies[body_handle];
         body.set_translation(player.checkpoint.nalgebra(), true);
         respawn.push(id);
      }
      for player in respawn {
         let _ = world.remove_one::<Dead>(player);
         world.get_mut::<Player>(player).unwrap().start_spawn_animation();
      }

      // Check if any of the players is touching a morph zone.
      let mut morphs = Vec::new();
      for (id, (_, morph, &RigidBody(body_handle), &Collider(collider_handle))) in
         world.query_mut::<(&mut Player, &mut Morph, &RigidBody, &Collider)>()
      {
         let body = &physics.rigid_bodies[body_handle];
         let collider = &physics.colliders[collider_handle];
         let zone_morph = if let Some(zone_collider_handle) = physics.query.intersection_with_shape(
            &physics.colliders,
            body.position(),
            collider.shape(),
            InteractionGroups::new(CollisionGroups::PLAYER, CollisionGroups::MORPH_ZONES),
            None,
         ) {
            let zone_collider = &physics.colliders[zone_collider_handle];
            Morph::FROM_U8[zone_collider.user_data as usize]
         } else {
            Morph::Unshaped
         };
         if zone_morph != *morph {
            morphs.push((id, zone_morph));
            *morph = zone_morph;
         }
      }
      for (player, morph) in morphs {
         Self::morph(world, player, morph);
         world.get_mut::<Player>(player).unwrap().start_spawn_animation();
      }
   }

   /// Morps the given player into the given morph kind.
   fn morph(world: &mut World, player: Entity, morph: Morph) {
      // Clear all existing morphs.
      let _ = world.remove_one::<Unshaped>(player);
      let _ = world.remove_one::<Platformer>(player);
      // Add the appropriate one given the kind.
      let _ = match morph {
         Morph::None => unreachable!(),
         Morph::Unshaped => world.insert_one(player, Unshaped::new()),
         Morph::Platformer => world.insert_one(player, Platformer::new()),
      };
   }

   /// Draws players.
   pub fn draw(ctx: &mut Context, world: &mut World) -> anyhow::Result<()> {
      for (_id, (player, platformer, InterpolatedPosition(position), &Size(size))) in
         world.query_mut::<Alive<(&Player, &Platformer, &InterpolatedPosition, &Size)>>()
      {
         let position = position.blend(ctx);
         let size = size * player.spawn_animation.get();
         let aspect = platformer.aspect_ratio.get();
         let stretched_squished = vector(aspect * size.y, size.x / aspect);
         let rect = rect(
            position
               + vector(
                  -stretched_squished.x / 2.0,
                  -stretched_squished.y + size.y / 2.0,
               ),
            stretched_squished,
         );
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

      let entity = world.spawn((
         Player::new(position),
         Position(position),
         InterpolatedPosition::new(position),
         Size(size),
         RigidBody(body),
         Collider(collider),
         Morph::None,
         Camera::new(),
      ));
      // Make sure the camera is initialized to the player's viewport, to prevent jank.
      physics.update_query_pipeline();
      Camera::warp(world, physics, entity);
      Self::tick(world, physics);
      entity
   }
}
