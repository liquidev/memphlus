//! Component for marking dead entities, and utilities related to that.

use hecs::{Without, World};

/// Marker component for keeping the entity alive for some ticks, eg. to perform an animation.
pub struct Kill {
   /// The remaining number of ticks before an entity is made dead.
   ///
   /// Note that when this is 0, the entity will not be killed instantly; rather, one tick will pass
   /// and the entity will _then_ be killed. Thus, a remaining time of 0 functions the same as 1.
   pub remaining_time: usize,
}

impl Kill {
   /// Creates a component that kills an entity after an amount of ticks has passed.
   pub fn after(n_ticks: usize) -> Kill {
      Kill {
         remaining_time: n_ticks,
      }
   }

   /// Ticks to-be-killed entities.
   pub fn tick(world: &mut World) {
      let mut killed = Vec::new();
      for (id, kill) in world.query::<&mut Kill>().iter() {
         kill.remaining_time = kill.remaining_time.saturating_sub(1);
         if kill.remaining_time <= 0 {
            killed.push(id);
         }
      }
      for entity in killed {
         // Ignore errors here, as they are not fatal.
         let _ = world.remove_one::<Kill>(entity);
         let _ = world.insert_one(entity, Dead);
      }
   }
}

/// Marker component signifying that an entity is dead and should not be rendered, nor ticked.
pub struct Dead;

/// A query for entities that are not to-be-killed ([`Kill`]) nor already [`Dead`].
pub type Alive<T> = Without<Kill, Without<Dead, T>>;
