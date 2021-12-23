//! Camera views and moving the camera to the player's position.

use hecs::{Entity, World};
use rapier2d::prelude::{Ball, ColliderBuilder, InteractionGroups};
use tetra::Context;
use vek::Vec2;

use crate::common::{rect, vector, Rect, ToNalgebraVector2};
use crate::interpolation::Interpolated;
use crate::physics::{CollisionGroups, Physics};

use super::physics::{Collider, RigidBody};
use super::{Position, Size};

/// Marker component for signifying that an entity is a camera view.
pub struct CameraView;

impl CameraView {
   /// Spawns a new camera view into the world.
   pub fn spawn(world: &mut World, physics: &mut Physics, entity: Entity, rect: Rect) {
      let collider = ColliderBuilder::cuboid(rect.width / 2.0, rect.height / 2.0)
         .translation(rect.center().nalgebra())
         .collision_groups(InteractionGroups::new(
            CollisionGroups::CAMERA_VIEWS,
            CollisionGroups::PLAYER,
         ))
         .build();
      let collider = physics.colliders.insert(collider);
      world.spawn_at(
         entity,
         (
            CameraView,
            Position(rect.top_left()),
            Size(vector(rect.width, rect.height)),
            Collider(collider),
         ),
      );
      // The entity ID of the camera view is accessible through the collider's user data.
      physics.colliders[collider].user_data = u64::from(entity.to_bits()) as u128;
   }
}

/// Camera state component.
#[derive(Debug, Clone)]
pub struct Camera {
   /// The view this camera is looking at.
   pub view: Option<Entity>,
   /// The center of the camera's viewport.
   pub position: Interpolated<Vec2<f32>>,
   /// The size of the camera's viewport.
   pub size: Interpolated<Vec2<f32>>,
}

impl Camera {
   /// Creates a new camera component.
   pub fn new() -> Self {
      Self {
         view: None,
         position: Interpolated::new(Default::default()),
         size: Interpolated::new(Default::default()),
      }
   }

   /// Clones the camera component out of the given entity and returns it.
   pub fn get(world: &mut World, entity: Entity) -> Camera {
      let camera = world.get_mut::<Camera>(entity).unwrap();
      camera.clone()
   }

   /// Warps the entity's camera to the current view it's standing in, instantly.
   pub fn warp(world: &mut World, physics: &mut Physics, entity: Entity) {
      Self::update_current_view(world, physics);
      let mut camera = world.get_mut::<Camera>(entity).unwrap();
      if let Some(view) = camera.view {
         let mut view_data = world.query_one::<(&Position, &Size)>(view).unwrap();
         let (&Position(target_position), &Size(target_size)) = view_data.get().unwrap();
         camera.position.set(target_position);
         camera.position.reset();
         camera.size.set(target_size);
         camera.size.reset();
      }
   }

   /// Updates the current views of all cameras.
   fn update_current_view(world: &mut World, physics: &mut Physics) {
      for (_id, (camera, &RigidBody(body_handle), &Collider(collider_handle))) in
         world.query_mut::<(&mut Camera, &RigidBody, &Collider)>()
      {
         // First, assume there is no view we're currently intersecting.
         camera.view = None;

         // Then, find out whether there actually _is_ such a view.
         let body = &mut physics.rigid_bodies[body_handle];
         let groups = physics.colliders[collider_handle].collision_groups();
         if let Some(collider) = physics.query.intersection_with_shape(
            &physics.colliders,
            body.position(),
            &Ball::new(0.01),
            InteractionGroups::new(groups.memberships, CollisionGroups::CAMERA_VIEWS),
            None,
         ) {
            let collider = &physics.colliders[collider];
            let view = Entity::from_bits(collider.user_data as u64).unwrap();
            camera.view = Some(view);
         }
      }
   }

   /// Ticks cameras attached to entities.
   pub fn tick(world: &mut World, physics: &mut Physics) {
      Self::update_current_view(world, physics);

      // Once we do find out whether there's a camera view, animate the position and size update.
      const ANIMATION_SPEED: f32 = 0.20;
      for (_id, camera) in world.query::<&mut Camera>().iter() {
         if let Some(view) = camera.view {
            let mut view_data = world.query_one::<(&Position, &Size)>(view).unwrap();
            let (&Position(target_position), &Size(target_size)) = view_data.get().unwrap();
            camera.position.update(Vec2::lerp(
               camera.position.current(),
               target_position,
               ANIMATION_SPEED,
            ));
            camera.size.update(Vec2::lerp(
               camera.size.current(),
               target_size,
               ANIMATION_SPEED,
            ));
         }
      }
   }

   /// Linearly interpolates the camera's position and size.
   pub fn blend(&self, ctx: &Context) -> Rect {
      let position = self.position.blend(ctx);
      let size = self.size.blend(ctx);
      rect(position, size)
   }
}
