//! Minimal Tiled JSON loader.

use std::collections::HashMap;
use std::ops::Deref;

use serde::de::Visitor;
use serde::Deserialize;

pub type TileId = u16;

pub type ObjectId = u32;

/// The value of a property.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum PropertyValue {
   String(String),
   Int(i32),
   Float(f32),
   Bool(bool),
   Object(u32),
}

/// Storage for properties.
#[derive(Debug, Clone)]
pub struct Properties(HashMap<String, PropertyValue>);

/// Properties dereference to a `HashMap<String, PropertyValue>`.
impl Deref for Properties {
   type Target = HashMap<String, PropertyValue>;

   fn deref(&self) -> &Self::Target {
      &self.0
   }
}

impl<'de> Deserialize<'de> for Properties {
   fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
   where
      D: serde::Deserializer<'de>,
   {
      #[derive(Debug, Deserialize)]
      struct Property {
         name: String,
         #[serde(flatten)]
         value: PropertyValue,
      }

      struct PropertyVisitor {
         properties: HashMap<String, PropertyValue>,
      }

      impl<'de> Visitor<'de> for PropertyVisitor {
         type Value = HashMap<String, PropertyValue>;

         fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("property array")
         }

         fn visit_seq<A>(mut self, mut seq: A) -> Result<Self::Value, A::Error>
         where
            A: serde::de::SeqAccess<'de>,
         {
            while let Some(item) = seq.next_element::<Property>()? {
               self.properties.insert(item.name, item.value);
            }
            Ok(self.properties)
         }
      }

      deserializer
         .deserialize_seq(PropertyVisitor {
            properties: HashMap::new(),
         })
         .map(|hash_map| Properties(hash_map))
   }
}

impl Default for Properties {
   fn default() -> Self {
      Self(Default::default())
   }
}

/// A tile specification - its ID and kind, as specified in the editor.
#[derive(Debug, Clone, Deserialize)]
pub struct Tile {
   /// The unique ID of the tile.
   pub id: TileId,
   /// The kind of tile, as specified in the editor.
   #[serde(rename = "type")]
   pub kind: String,
   /// The objects that make up the tile's collision.
   #[serde(rename = "objectgroup")]
   pub object_group: Option<LayerKind>,
   /// The custom properties of the tile.
   #[serde(default)]
   pub properties: Properties,
}

/// A tileset.
#[derive(Debug, Clone, Deserialize)]
pub struct Tileset {
   /// The width of tiles in the tileset.
   #[serde(rename = "tilewidth")]
   pub tile_width: u32,
   /// The height of tiles in the tileset.
   #[serde(rename = "tileheight")]
   pub tile_height: u32,

   /// The total number of tiles in the tileset. This is not the same as `tiles.len()`, because
   /// this is `maximum_tile_id + 1`. Might be convenient for allocations.
   #[serde(rename = "tilecount")]
   pub tile_count: u32,
   pub tiles: Vec<Tile>,
}

impl Tileset {
   /// Loads a tileset from JSON data.
   pub fn load_from_json(json: &str) -> anyhow::Result<Self> {
      Ok(serde_json::from_str(json)?)
   }
}

/// A chunk of tiles.
#[derive(Debug, Clone, Deserialize)]
pub struct Chunk {
   pub data: Vec<TileId>,
   pub x: i32,
   pub y: i32,
}

/// An object in an object layer.
#[derive(Debug, Clone, Deserialize)]
pub struct Object {
   pub id: ObjectId,
   pub x: f32,
   pub y: f32,
   pub width: f32,
   pub height: f32,
   pub rotation: f32,
   #[serde(rename = "type")]
   pub kind: String,
   #[serde(default)]
   pub properties: Properties,

   /// If `Some`, the object is a text object.
   pub text: Option<Text>,
}

/// The horizontal alignment of text inside a text object.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TextHAlign {
   Left,
   Center,
   Right,
}

impl Default for TextHAlign {
   fn default() -> Self {
      Self::Left
   }
}

/// A text object.
#[derive(Debug, Clone, Deserialize)]
pub struct Text {
   #[serde(rename = "fontfamily")]
   pub font_family: String,
   #[serde(rename = "halign", default)]
   pub h_align: TextHAlign,
   #[serde(rename = "pixelsize")]
   pub pixel_size: u32,
   pub text: String,
}

/// The kind of a layer.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum LayerKind {
   #[serde(rename = "tilelayer")]
   Tile { chunks: Vec<Chunk> },
   #[serde(rename = "objectgroup")]
   Object { objects: Vec<Object> },
}

/// A layer.
#[derive(Debug, Clone, Deserialize)]
pub struct Layer {
   #[serde(flatten)]
   pub kind: LayerKind,
}

/// A Tiled map.
#[derive(Debug, Clone, Deserialize)]
pub struct Map {
   pub layers: Vec<Layer>,
}

impl Map {
   /// Loads a map from JSON data.
   pub fn load_from_json(json: &str) -> anyhow::Result<Self> {
      Ok(serde_json::from_str(json)?)
   }
}
