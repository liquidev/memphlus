//! Loading of entities.

use std::str::FromStr;

use hecs::{Entity, World};
use serde::de::IntoDeserializer;
use serde::Deserialize;
use vek::Mat2;

use crate::common::{rect, vector};
use crate::entities::camera::CameraView;
use crate::entities::colliders::RectCollider;
use crate::entities::player::Player;
use crate::entities::zones::{DeadlyZone, PlatformerZone, ZoneIndex, ZoneSpawn, Zones};
use crate::physics::Physics;
use crate::tiled;

use super::{Layer, Map};

/// Viable entity kinds, as stored in the map.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
enum EntityKind {
   Player,
   Collider,
   CameraView,

   ZonePlatformer,
   ZoneDeadly,
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
            Self::spawn_entity(kind, object, world, physics);
         } else {
            eprintln!("warning: object of unknown kind {:?}", &object.kind);
         }
      }
      Layer::Object
   }

   /// Spawns an entity of the given kind into the world.
   fn spawn_entity(
      kind: EntityKind,
      data: tiled::Object,
      world: &mut World,
      physics: &mut Physics,
   ) {
      let data = tiled::Object {
         x: data.x / Map::tile_size().x,
         y: data.y / Map::tile_size().y,
         width: data.width / Map::tile_size().x,
         height: data.height / Map::tile_size().y,
         rotation: data.rotation / 180.0 * std::f32::consts::PI,
         ..data
      };
      let position = vector(data.x, data.y);
      let size = vector(data.width, data.height);
      let rect = rect(position, size);
      let _ = match kind {
         EntityKind::Player => Player::spawn(world, physics, position),
         EntityKind::Collider => Self::spawn_collider(&data, world, physics),
         EntityKind::CameraView => CameraView::spawn(world, physics, rect),
         EntityKind::ZoneDeadly => Self::spawn_zone(&data, world, physics, DeadlyZone),
         EntityKind::ZonePlatformer => Self::spawn_zone(&data, world, physics, PlatformerZone),
      };
   }

   /// Spawns an appropriate collider entity.
   fn spawn_collider(data: &tiled::Object, world: &mut World, physics: &mut Physics) -> Entity {
      let position = vector(data.x, data.y);
      let size = vector(data.width, data.height);
      RectCollider::spawn(world, physics, rect(position, size))
   }

   /// Spawns a zone of the given kind.
   fn spawn_zone(
      data: &tiled::Object,
      world: &mut World,
      physics: &mut Physics,
      kind: impl ZoneIndex + ZoneSpawn,
   ) -> Entity {
      let top_left = vector(data.x, data.y);
      let size = vector(data.width, data.height);
      let center_offset = size / 2.0;
      let rotation = Mat2::rotation_z(data.rotation);
      let center = top_left + rotation * center_offset;
      Zones::spawn(world, physics, kind, center, size, data.rotation)
   }
}
