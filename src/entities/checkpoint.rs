//! Externally triggered checkpoints.

use hecs::{Entity, World};
use vek::Vec2;

use super::trigger::Triggered;
use super::Position;

/// A respawn position. If an entity triggers a checkpoint and has the `RespawnPosition` component,
/// that position will be set to the checkpoint's position.
pub struct RespawnPosition(pub Vec2<f32>);

/// Marker struct specifying that an entity is a checkpoint.
pub struct Checkpoint;

impl Checkpoint {
   /// Trigger method used for setting an entity's respawn position.
   pub const SET_RESPAWN_POSITION: u32 = 0;

   /// Ticks checkpoints.
   pub fn tick(world: &mut World) {
      let mut set_respawns = Vec::new();
      for (id, (&Checkpoint, triggered)) in world.query_mut::<(&Checkpoint, &Triggered)>() {
         if triggered.method == Self::SET_RESPAWN_POSITION {
            set_respawns.push((id, triggered.source));
         }
      }
      for (checkpoint, entity) in set_respawns {
         {
            let Position(position) = *world.get(checkpoint).unwrap();
            let RespawnPosition(respawn_position) = &mut *world.get_mut(entity).unwrap();
            *respawn_position = position;
         }
         let _ = world.remove_one::<Triggered>(checkpoint);
      }
   }

   /// Spawns a checkpoint at the given entity.
   pub fn spawn(world: &mut World, entity: Entity, position: Vec2<f32>) {
      world.spawn_at(entity, (Checkpoint, Position(position)));
   }
}
