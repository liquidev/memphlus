//! Loading of entities.

use std::str::FromStr;

use hecs::{Entity, World};
use log::{error, warn};
use serde::de::IntoDeserializer;
use serde::Deserialize;
use vek::Mat2;

use crate::assets::FontFamily;
use crate::common::{rect, vector, Rect};
use crate::entities::camera::CameraView;
use crate::entities::colliders::RectCollider;
use crate::entities::player::Player;
use crate::entities::text::Text;
use crate::entities::zones::{DeadlyZone, PlatformerZone, ZoneData, ZoneSpawn, Zones};
use crate::physics::Physics;
use crate::tiled;

use super::{Layer, Map};

/// Viable entity kinds, as stored in the map.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
enum EntityKind {
   Player,
   Text,

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
            warn!("object {} of unknown kind {:?}", object.id, &object.kind);
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
      match kind {
         EntityKind::Player => {
            Player::spawn(world, physics, position);
         }
         EntityKind::Text => Self::spawn_text(data, world),
         EntityKind::Collider => Self::spawn_collider(&data, world, physics),
         EntityKind::CameraView => {
            CameraView::spawn(world, physics, rect);
         }
         EntityKind::ZoneDeadly => Self::spawn_zone(&data, world, physics, DeadlyZone),
         EntityKind::ZonePlatformer => Self::spawn_zone(&data, world, physics, PlatformerZone),
      };
   }

   /// Spawns text into the world.
   fn spawn_text(data: tiled::Object, world: &mut World) {
      let rect = data.rect();
      if let Some(text) = data.text {
         match FontFamily::from_str(&text.font_family) {
            Ok(font_family) => {
               Text::spawn(world, rect, font_family, text.h_align, text.text);
            }
            Err(error) => error!("object {}: invalid font family ({})", data.id, error),
         }
      } else {
         error!("object {} of type 'text' is not a text object", data.id);
      }
   }

   /// Spawns an appropriate collider entity.
   fn spawn_collider(data: &tiled::Object, world: &mut World, physics: &mut Physics) {
      RectCollider::spawn(world, physics, data.rect());
   }

   /// Spawns a zone of the given kind.
   fn spawn_zone(
      data: &tiled::Object,
      world: &mut World,
      physics: &mut Physics,
      kind: impl ZoneData + ZoneSpawn,
   ) {
      let top_left = vector(data.x, data.y);
      let size = vector(data.width, data.height);
      let center_offset = size / 2.0;
      let rotation = Mat2::rotation_z(data.rotation);
      let center = top_left + rotation * center_offset;
      Zones::spawn(world, physics, kind, center, size, data.rotation);
   }
}

impl tiled::Object {
   fn rect(&self) -> Rect {
      rect(vector(self.x, self.y), vector(self.width, self.height))
   }
}
