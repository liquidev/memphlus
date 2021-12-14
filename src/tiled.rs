//! Minimal Tiled JSON loader.

use serde::{Deserialize, Serialize};

/// A tile specification - its ID and kind, as specified in the editor.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Tile {
   /// The unique ID of the tile.
   pub id: u32,
   /// The kind of tile, as specified in the editor.
   #[serde(rename = "type")]
   pub kind: String,
}

/// A tileset.
#[derive(Debug, Clone, Deserialize, Serialize)]
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
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Chunk {
   pub data: Vec<u32>,
   pub x: u32,
   pub y: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Object {
   pub x: f32,
   pub y: f32,
   pub width: f32,
   pub height: f32,
   #[serde(rename = "type")]
   pub kind: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum LayerKind {
   #[serde(rename = "tilelayer")]
   Tile { chunks: Vec<Chunk> },
   #[serde(rename = "objectgroup")]
   Object { objects: Vec<Object> },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Layer {
   #[serde(flatten)]
   pub kind: LayerKind,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Map {
   pub layers: Vec<Layer>,
}

impl Map {
   /// Loads a map from JSON data.
   pub fn load_from_json(json: &str) -> anyhow::Result<Self> {
      Ok(serde_json::from_str(json)?)
   }
}
