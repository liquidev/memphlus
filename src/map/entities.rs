//! Loading of entities.

use std::str::FromStr;

use hecs::World;
use serde::de::IntoDeserializer;
use serde::Deserialize;

use crate::common::vector;
use crate::entities::player::Player;
use crate::physics::Physics;
use crate::tiled;

use super::{Layer, Map};

/// Viable entity kinds, as stored in the map.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
enum EntityKind {
   Player,
}

impl FromStr for EntityKind {
   type Err = serde::de::value::Error;

   fn from_str(s: &str) -> Result<Self, Self::Err> {
      Self::deserialize(s.into_deserializer())
   }
}

impl Map {
   /// Creates an object layer.
   pub(super) fn create_object_layer(
      objects: Vec<tiled::Object>,
      world: &mut World,
      physics: &mut Physics,
   ) -> Layer {
      for object in objects {
         if let Ok(kind) = EntityKind::from_str(&object.kind) {
            Self::spawn_entity(kind, &object, world, physics);
         } else {
            eprintln!("warning: object of unknown kind {:?}", &object.kind);
         }
      }
      Layer::Object
   }

   /// Spawns an entity of the given kind into the world.
   fn spawn_entity(
      kind: EntityKind,
      data: &tiled::Object,
      world: &mut World,
      physics: &mut Physics,
   ) {
      let position = vector(data.x, data.y) / Self::tile_size();
      let _ = match kind {
         EntityKind::Player => Player::spawn(world, physics, position),
      };
   }
}
