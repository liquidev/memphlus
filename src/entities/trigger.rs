//! Trigger entities.

use hecs::{CommandBuffer, Entity, World};
use rapier2d::prelude::{ColliderBuilder, InteractionGroups};

use crate::common::{Rect, RectVectors, ToNalgebraVector2};
use crate::physics::{CollisionGroups, Physics};

use super::physics::Collider;
use super::{Position, Size};

/// Component added to trigger targets when triggers linking to them are activated.
pub struct Triggered {
   /// Which method was triggered.
   pub method: u32,
   /// Which object triggered the method.
   pub source: Entity,
   /// Which trigger triggered the method.
   pub trigger: Entity,
}

/// A marker component for marking that an entity is a collision trigger.
pub struct Trigger {
   /// Which object this trigger triggers.
   pub target: Entity,
   /// Which method this trigger should activate.
   pub method: u32,
   triggered_this_tick: bool,
}

impl Trigger {
   /// Creates a new trigger component.
   pub fn new(target: Entity, method: u32) -> Self {
      Self {
         target,
         method,
         triggered_this_tick: false,
      }
   }

   /// Ticks all triggers in the world.
   pub fn tick(world: &mut World, physics: &mut Physics) {
      let mut triggers = CommandBuffer::new();
      for (id, (trigger, &Collider(collider_handle))) in
         world.query_mut::<(&mut Trigger, &Collider)>()
      {
         let collider = &physics.colliders[collider_handle];
         if let Some(other_collider_handle) = physics.query.intersection_with_shape(
            &physics.colliders,
            collider.position(),
            collider.shape(),
            collider.collision_groups(),
            None,
         ) {
            if !trigger.triggered_this_tick {
               let other_collider = &physics.colliders[other_collider_handle];
               let entity = Entity::from_bits(other_collider.user_data as u64).unwrap();
               triggers.insert(
                  trigger.target,
                  (Triggered {
                     method: trigger.method,
                     source: entity,
                     trigger: id,
                  },),
               );
               trigger.triggered_this_tick = true;
            }
         } else {
            trigger.triggered_this_tick = false;
         }
      }
      triggers.run_on(world);
   }

   /// Spawns a trigger entity into the world.
   pub fn spawn(
      world: &mut World,
      physics: &mut Physics,
      entity: Entity,
      rect: Rect,
      trigger: Trigger,
   ) {
      let half_extents = rect.size() / 2.0;
      let collider = ColliderBuilder::cuboid(half_extents.x, half_extents.y)
         .translation(rect.center().nalgebra())
         .collision_groups(InteractionGroups::new(
            CollisionGroups::TRIGGERS,
            CollisionGroups::PLAYER,
         ))
         .sensor(true)
         .build();
      let collider_handle = physics.colliders.insert(collider);

      world.spawn_at(
         entity,
         (
            trigger,
            Position(rect.position()),
            Size(rect.size()),
            Collider(collider_handle),
         ),
      )
   }
}
