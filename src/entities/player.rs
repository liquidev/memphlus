//! Components and systems for the player entity.

use std::time::Duration;

use hecs::{Component, Entity, World};
use rapier2d::math::Isometry;
use rapier2d::prelude::{
   Ball, CoefficientCombineRule, ColliderBuilder, Cuboid, InteractionGroups, RigidBodyBuilder,
   RigidBodyHandle, SharedShape,
};
use tetra::graphics::mesh::{GeometryBuilder, ShapeStyle};
use tetra::graphics::DrawParams;
use tetra::math::Vec2;
use tetra::Context;

use crate::assets::RemappableColors;
use crate::common::{rect, stretch_squish, vector, ToNalgebraVector2, ToVekVec2};
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

/// Parameters for body physics.
struct PhysicsParams {
   size: Vec2<f32>,
   shape: SharedShape,
   restitution: f32,
   gravity_scale: f32,
}

/// Component for storing state of unshaped controls.
pub struct Unshaped {}

impl Unshaped {
   const RADIUS: f32 = 0.3;

   pub fn new() -> Self {
      Self {}
   }

   fn size() -> Vec2<f32> {
      vector(Self::RADIUS, Self::RADIUS) * 2.0
   }

   /// Returns the physics parameters for the unshaped body.
   fn physics_params() -> PhysicsParams {
      PhysicsParams {
         size: Self::size(),
         shape: SharedShape::new(Ball::new(Self::RADIUS)),
         restitution: 0.8,
         gravity_scale: 0.0,
      }
   }

   fn tick_controls(world: &mut World, physics: &mut Physics, input: &Input) {
      const ACCELERATION: f32 = 50.0;
      const DAMPING: f32 = 0.97;

      for (_id, (_, _, &RigidBody(body_handle))) in
         world.query_mut::<Alive<(&Player, &Unshaped, &RigidBody)>>()
      {
         let body = &mut physics.rigid_bodies[body_handle];
         if input.joystick().magnitude_squared() > 0.3 * 0.3 {
            body.apply_force((input.joystick() * ACCELERATION).nalgebra(), true);
         } else {
            let velocity = body.linvel().vek();
            let dampened = velocity * DAMPING;
            body.set_linvel(dampened.nalgebra(), true);
         }
      }
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
   const SIZE: [f32; 2] = [0.8, 0.8];

   pub fn new() -> Self {
      Self {
         remaining_jump_ticks: 0,
         jump_buffer: 0,
         air_time: 0,
         aspect_ratio: Tween::new(1.0),
         previous_velocity: vector(0.0, 0.0),
      }
   }

   /// Returns the size of the platformer body as a vector.
   fn size() -> Vec2<f32> {
      Vec2::from_slice(&Self::SIZE)
   }

   /// Initializes a rigid body and collider for platformer controls.
   fn physics_params() -> PhysicsParams {
      PhysicsParams {
         size: Self::size(),
         shape: SharedShape::new(Cuboid::new((Self::size() / 2.0).nalgebra())),
         restitution: 0.0,
         gravity_scale: 1.0,
      }
   }

   /// Ticks the player controls.
   fn tick_controls(ctx: &mut Context, world: &mut World, physics: &mut Physics, input: &Input) {
      const ACCELERATION: f32 = 175.0;
      const DECELERATION: f32 = 0.8;
      const JUMP_STRENGTH: f32 = 700.0;
      // The number of ticks during which the jump button can be held down to adjust height.
      const JUMP_SUSTAIN: u8 = 10;
      // The number of ticks of leeway during which one can be falling down and still jump when
      // they land.
      const JUMP_LEEWAY: u8 = 8;
      // The number of ticks during which you can still jump after falling down a ledge.
      const COYOTE_TIME: u8 = 10;
      // The maximum velocity at which moving around is considered "walking", that is, the player
      // retains control of their walking direction.
      const WALKING_VELOCITY: f32 = 12.0;

      for (_id, (_, platformer, &RigidBody(body_handle))) in
         world.query_mut::<Alive<(&Player, &mut Platformer, &RigidBody)>>()
      {
         let is_walking = {
            let body = &mut physics.rigid_bodies[body_handle];
            let velocity = body.linvel().vek();
            let is_walking = velocity.x.abs() < WALKING_VELOCITY;

            if is_walking && input.joystick().x.abs() > 0.3 {
               body.apply_force(
                  vector(ACCELERATION * input.joystick().x, 0.0).nalgebra(),
                  true,
               );
            }

            is_walking
         };

         let is_on_ground = Self::is_on_ground(physics, body_handle);
         if input.button_just_pressed(ctx, Button::Jump) {
            platformer.jump_buffer = JUMP_LEEWAY;
         }
         if is_on_ground {
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

         let mut velocity = body.linvel().vek();
         velocity.x *= if is_walking || is_on_ground {
            DECELERATION
         } else {
            1.0
         };
         body.set_linvel(velocity.nalgebra(), true);

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
      let half_extents = Vec2::from_slice(&Platformer::SIZE) / 2.0;
      let collider_size = vector(half_extents.x - 0.05, 0.0);
      let cuboid = Cuboid::new(collider_size.nalgebra());
      let translation = body.translation();
      let translation = vector(
         translation.x,
         translation.y + half_extents.y - collider_size.y,
      );
      // TODO(liquidev): wtf double jump is possible for some reason if you tap space quickly
      physics
         .query
         .intersection_with_shape(
            &physics.colliders,
            &Isometry::new(translation.nalgebra(), 0.0),
            &cuboid,
            InteractionGroups::new(CollisionGroups::PLAYER, CollisionGroups::SOLIDS),
            None,
         )
         .is_some()
   }
}

/// Marker component and namespace for player-related functions.
pub struct Player {
   checkpoint: Vec2<f32>,
   /// Animation triggered right when the player spawns in, or turns into a different morph.
   spawn_animation: Tween<f32>,
}

impl Player {
   /// Creates a new player with the provided initial checkpoint.
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

   /// Ticks the players' controls.
   pub fn tick_controls(
      ctx: &mut Context,
      world: &mut World,
      physics: &mut Physics,
      input: &Input,
   ) {
      Unshaped::tick_controls(world, physics, input);
      Platformer::tick_controls(ctx, world, physics, input);
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
      for (id, (player, InterpolatedPosition(ip), &RigidBody(body_handle), &Dead)) in
         world.query_mut::<(&Player, &mut InterpolatedPosition, &RigidBody, &Dead)>()
      {
         let body = &mut physics.rigid_bodies[body_handle];
         // Prevent interpolation jank by resetting the current and previous position to the
         // same value.
         ip.set(player.checkpoint);
         ip.reset();
         body.set_translation(player.checkpoint.nalgebra(), true);
         // TODO(liquidev): Velocity-preserving death might be a neat mechanic.
         body.set_linvel(vector(0.0, 0.0).nalgebra(), true);
         respawn.push(id);
      }
      for player in respawn {
         let _ = world.remove_one::<Dead>(player);
         world.get_mut::<Player>(player).unwrap().start_spawn_animation();
      }

      // Check if any of the players is touching a morph zone.
      let mut morphs = Vec::new();
      for (id, (_, morph, &RigidBody(body_handle))) in
         world.query_mut::<(&mut Player, &mut Morph, &RigidBody)>()
      {
         let body = &physics.rigid_bodies[body_handle];
         let zone_morph = if let Some(zone_collider_handle) = physics.query.intersection_with_shape(
            &physics.colliders,
            body.position(),
            &Ball::new(0.01),
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
         Self::morph(world, physics, player, morph);
         world.get_mut::<Player>(player).unwrap().start_spawn_animation();
      }
   }

   /// Morps the given player into the given morph kind.
   fn morph(world: &mut World, physics: &mut Physics, player: Entity, morph: Morph) {
      // Clear all existing morphs.
      let _ = world.remove_one::<Unshaped>(player);
      let _ = world.remove_one::<Platformer>(player);
      // Add the appropriate one given the kind.
      let _ = match morph {
         Morph::None => panic!("Player::morph called with None morph type"),
         Morph::Unshaped => world.insert_one(player, Unshaped::new()),
         Morph::Platformer => world.insert_one(player, Platformer::new()),
      };
      // Update the physics properties.
      let body_handle = world.get::<RigidBody>(player).unwrap().0;
      let body = &mut physics.rigid_bodies[body_handle];
      let collider_handle = world.get::<Collider>(player).unwrap().0;
      let collider = &mut physics.colliders[collider_handle];
      let params = match morph {
         Morph::None => unreachable!(),
         Morph::Unshaped => Unshaped::physics_params(),
         Morph::Platformer => Platformer::physics_params(),
      };
      collider.set_shape(params.shape);
      collider.set_restitution(params.restitution);
      body.set_gravity_scale(params.gravity_scale, true);
      world.get_mut::<Size>(player).unwrap().0 = params.size;
   }

   /// Draws a morph with the state `M`.
   fn draw_morph<M, F>(ctx: &mut Context, world: &mut World, mut draw: F) -> anyhow::Result<()>
   where
      M: Component,
      F: FnMut(&mut Context, &M, Vec2<f32>, Vec2<f32>, RigidBodyHandle) -> anyhow::Result<()>,
   {
      for (
         _id,
         (player, morph, InterpolatedPosition(position), &Size(size), &RigidBody(body_handle)),
      ) in world.query_mut::<Alive<(&Player, &M, &InterpolatedPosition, &Size, &RigidBody)>>()
      {
         let size = size * player.spawn_animation.get();
         draw(ctx, morph, position.blend(ctx), size, body_handle)?;
      }
      Ok(())
   }

   /// Draws players.
   pub fn draw(ctx: &mut Context, world: &mut World, physics: &mut Physics) -> anyhow::Result<()> {
      Self::draw_morph::<Unshaped, _>(
         ctx,
         world,
         |ctx, _unshaped, position, size, body_handle| {
            const SCALE: f32 = 8.0;
            const INV_SCALE: f32 = 1.0 / SCALE;
            let velocity = physics.rigid_bodies[body_handle].linvel().vek();
            let rotation = velocity.y.atan2(velocity.x);
            let aspect = 1.0 + velocity.magnitude_squared() * 0.001;
            let stretched_squished = stretch_squish(size / 2.0, aspect);
            // TODO(liquidev): Metaballs.
            GeometryBuilder::new()
               .set_color(RemappableColors::BACKGROUND)
               .ellipse(
                  ShapeStyle::Fill,
                  vector(0.0, 0.0),
                  stretched_squished * SCALE,
               )?
               .set_color(RemappableColors::FOREGROUND)
               .ellipse(
                  ShapeStyle::Stroke(0.1 * SCALE),
                  vector(0.0, 0.0),
                  stretched_squished * SCALE,
               )?
               .build_mesh(ctx)?
               .draw(
                  ctx,
                  DrawParams::new()
                     .position(position)
                     .scale(vector(INV_SCALE, INV_SCALE))
                     .rotation(rotation),
               );
            Ok(())
         },
      )?;

      Self::draw_morph::<Platformer, _>(
         ctx,
         world,
         |ctx, platformer, position, size, _body_handle| {
            let aspect = platformer.aspect_ratio.get();
            let stretched_squished = stretch_squish(size, aspect);
            let rect = rect(
               position
                  + vector(
                     -stretched_squished.x / 2.0,
                     -stretched_squished.y + size.y / 2.0,
                  ),
               stretched_squished,
            );
            GeometryBuilder::new()
               .set_color(RemappableColors::BACKGROUND)
               .rectangle(ShapeStyle::Fill, rect)?
               .set_color(RemappableColors::FOREGROUND)
               .rectangle(ShapeStyle::Stroke(0.1), rect)?
               .build_mesh(ctx)?
               .draw(ctx, DrawParams::new());
            Ok(())
         },
      )?;

      Ok(())
   }

   /// Spawns a new player into the world.
   pub fn spawn(world: &mut World, physics: &mut Physics, entity: Entity, position: Vec2<f32>) {
      let size = Vec2::from_slice(&Platformer::SIZE);

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

      world.spawn_at(
         entity,
         (
            Player::new(position),
            Position(position),
            InterpolatedPosition::new(position),
            Size(size),
            RigidBody(body),
            Collider(collider),
            Morph::None,
            Camera::new(),
         ),
      );
      // Make sure the camera is initialized to the player's viewport, to prevent jank.
      physics.update_query_pipeline();
      Camera::warp(world, physics, entity);
      Self::tick(world, physics);
   }
}
