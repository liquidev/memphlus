//! Map loading, storage, and physics.

mod rendering;

use std::collections::HashMap;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

use anyhow::Context;
use arrayvec::ArrayVec;
use hecs::World;
use rapier2d::prelude::ColliderBuilder;
use serde::de::IntoDeserializer;
use serde::{Deserialize, Serialize};

use crate::physics::Physics;
use crate::tiled::{self, TileId};

/// A map tile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[repr(u8)]
#[serde(rename_all = "snake_case")]
pub enum TileKind {
   /// The empty tile.
   Empty,

   // Solid blocks
   SolidTopLeft,
   SolidTop,
   SolidTopRight,
   SolidRight,
   SolidBottomRight,
   SolidBottom,
   SolidBottomLeft,
   SolidLeft,
   SolidVTop,
   SolidVMiddle,
   SolidVBottom,
   SolidHLeft,
   SolidHCenter,
   SolidHRight,
   SolidTile,
   SolidTopFadeLeft,
   SolidTopFadeRight,
   SolidBottomFadeLeft,
   SolidBottomFadeRight,
   SolidLeftFadeTop,
   SolidLeftFadeBottom,
   SolidRightFadeTop,
   SolidRightFadeBottom,
   Barrier,

   // Slopes
   SlopeUp,               // y = x
   SlopeDown,             // y = -x
   SlopeHalfUpLeft,       // y = (1/2)x
   SlopeHalfUpRight,      // y = (1/2)x + 1/2
   SlopeHalfDownLeft,     // y = -(1/2)x + 1
   SlopeHalfDownRight,    // y = -(1/2)x + 1/2
   SlopeDoubleUpBottom,   // y = 2x
   SlopeDoubleUpTop,      // y = 2x - 1
   SlopeDoubleDownBottom, // y = -2x + 2
   SlopeDoubleDownTop,    // y = -2x + 1

   // Spikes
   // The name reflects the direction in which the spikes are pointing.
   SpikesUp,
   SpikesRight,
   SpikesDown,
   SpikesLeft,
}

impl FromStr for TileKind {
   type Err = serde::de::value::Error;

   fn from_str(s: &str) -> Result<Self, Self::Err> {
      Self::deserialize(s.into_deserializer())
   }
}

/// The tileset of a map.
pub struct Tileset {
   pub kinds: Vec<TileKind>,
   pub colliders: Vec<ArrayVec<ColliderBuilder, 8>>,
}

impl Tileset {
   /// Returns the kind of the given tile.
   pub fn kind(&self, tile_id: TileId) -> TileKind {
      self.kinds[tile_id as usize]
   }
}

impl TryFrom<tiled::Tileset> for Tileset {
   type Error = anyhow::Error;

   fn try_from(data: tiled::Tileset) -> Result<Self, Self::Error> {
      let n_tiles = data.tile_count as usize;
      let mut set = Self {
         kinds: Vec::from_iter(std::iter::repeat(TileKind::Empty).take(n_tiles)),
         colliders: Vec::from_iter(std::iter::repeat_with(|| ArrayVec::new()).take(n_tiles)),
      };

      for tile in data.tiles {
         let id = tile.id as usize;
         let kind = TileKind::from_str(&tile.kind).context("invalid tile type used")?;
         set.kinds[id] = kind;
      }

      Ok(set)
   }
}

/// A chunk of tiles.
pub struct Chunk {
   tiles: [TileId; Self::LENGTH],
}

impl Chunk {
   /// The size of a chunk.
   const SIZE_BITS: u32 = 3;
   pub const SIZE: usize = 1 << Self::SIZE_BITS;
   const LENGTH: usize = Self::SIZE * Self::SIZE;

   /// Creates a new chunk from a tile ID.
   pub fn from_tile_id(id: TileId) -> Self {
      Self {
         tiles: [id; Self::LENGTH],
      }
   }
}

/// Indexing for chunks, using `(X, Y)` coordinates.
impl Index<(usize, usize)> for Chunk {
   type Output = TileId;

   fn index(&self, index: (usize, usize)) -> &Self::Output {
      &self.tiles[index.0 + Self::SIZE * index.1]
   }
}

/// Mutable indexing for chunks, using `(X, Y)` coordinates.
impl IndexMut<(usize, usize)> for Chunk {
   fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
      &mut self.tiles[index.0 + Self::SIZE * index.1]
   }
}

/// A layer.
pub enum Layer {
   Tile { chunks: HashMap<(u32, u32), Chunk> },
   Object,
}

impl Layer {
   /// Returns whether the layer is a tile layer.
   pub fn is_tile_layer(&self) -> bool {
      matches!(self, Self::Tile { .. })
   }

   /// Returns the layer's chunks, if available.
   pub fn chunks(&self) -> Option<&HashMap<(u32, u32), Chunk>> {
      if let Self::Tile { chunks } = self {
         Some(chunks)
      } else {
         None
      }
   }
}

/// An in-game map.
pub struct Map {
   pub tileset: Tileset,
   pub layers: Vec<Layer>,
}

impl Map {
   /// Loads a map from tileset and map JSON data.
   pub fn load_into_world_from_json(
      world: &mut World,
      physics: &mut Physics,
      tileset_json: &str,
      map_json: &str,
   ) -> anyhow::Result<Self> {
      let tileset = tiled::Tileset::load_from_json(tileset_json)?;
      let tileset = Tileset::try_from(tileset)?;
      let map = tiled::Map::load_from_json(map_json)?;
      Ok(Self {
         tileset,
         layers: Self::load_layers(map.layers),
      })
   }

   /// Loads all layers from the given Vec.
   fn load_layers(layers: Vec<tiled::Layer>) -> Vec<Layer> {
      layers.into_iter().map(Self::load_layer).collect()
   }

   /// Loads a single tiled layer into an actual layer.
   fn load_layer(data: tiled::Layer) -> Layer {
      match data.kind {
         tiled::LayerKind::Tile { chunks } => Self::create_tile_layer(chunks),
         tiled::LayerKind::Object { objects } => Layer::Object,
      }
   }

   /// Creates a new tile layer from a list of chunks.
   fn create_tile_layer(in_chunks: Vec<tiled::Chunk>) -> Layer {
      let mut chunks = HashMap::new();

      for chunk_data in in_chunks {
         let chunk_position = (
            chunk_data.x >> Chunk::SIZE_BITS,
            chunk_data.y >> Chunk::SIZE_BITS,
         );
         let mut chunk = Chunk::from_tile_id(0);
         for y in 0..Chunk::SIZE {
            for x in 0..Chunk::SIZE {
               // Subtract 1 because empty tiles are represented as 0, but we already have an
               // empty tile at ID 0 anyways.
               chunk[(x, y)] = chunk_data.data[x + y * Chunk::SIZE].saturating_sub(1);
            }
         }
         chunks.insert(chunk_position, chunk);
      }

      Layer::Tile { chunks }
   }
}
