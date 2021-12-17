//! Map loading, storage, and physics.

mod collision;
mod entities;
mod meshes;
mod rendering;

use std::collections::HashMap;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

use anyhow::Context;
use arrayvec::ArrayVec;
use bitflags::bitflags;
use ggez::graphics::Mesh;
use glam::Vec2;
use hecs::World;
use rapier2d::prelude::{ColliderBuilder, ColliderHandle};
use serde::de::IntoDeserializer;
use serde::{Deserialize, Serialize};

use crate::common::{vector, Axis};
use crate::physics::Physics;
use crate::tiled::{self, TileId};

pub use meshes::*;

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

impl TileKind {
   /// Returns the sole side which this tile represents.
   pub fn side(&self) -> Option<Side> {
      match self {
         Self::SolidTop | Self::SolidTopFadeLeft | Self::SolidTopFadeRight => Some(Side::Top),
         Self::SolidRight | Self::SolidRightFadeTop | Self::SolidRightFadeBottom => {
            Some(Side::Right)
         }
         Self::SolidBottom | Self::SolidBottomFadeLeft | Self::SolidBottomFadeRight => {
            Some(Side::Bottom)
         }
         Self::SolidLeft | Self::SolidLeftFadeTop | Self::SolidLeftFadeBottom => Some(Side::Left),
         _ => None,
      }
   }

   /// Returns the side at which spikes are pointing.
   pub fn spike_direction(&self) -> Option<Side> {
      match self {
         Self::SpikesUp => Some(Side::Top),
         Self::SpikesRight => Some(Side::Right),
         Self::SpikesDown => Some(Side::Bottom),
         Self::SpikesLeft => Some(Side::Left),
         _ => None,
      }
   }
}

impl FromStr for TileKind {
   type Err = serde::de::value::Error;

   fn from_str(s: &str) -> Result<Self, Self::Err> {
      Self::deserialize(s.into_deserializer())
   }
}

/// The side of a tile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
   Top,
   Bottom,
   Left,
   Right,
}

impl Side {
   /// Returns the axis on which the side's outline is rendered.
   pub fn axis(&self) -> Axis {
      match self {
         Side::Top | Side::Bottom => Axis::X,
         Side::Left | Side::Right => Axis::Y,
      }
   }
}

bitflags! {
   pub struct Sides: u8 {
      const TOP = 0b0001;
      const BOTTOM = 0b0010;
      const LEFT = 0b0100;
      const RIGHT = 0b1000;
      const ALL = Self::TOP.bits | Self::BOTTOM.bits | Self::LEFT.bits | Self::RIGHT.bits;
   }
}

#[derive(Debug)]
pub struct NotAxisAligned;

impl TryFrom<TileKind> for Sides {
   type Error = NotAxisAligned;

   fn try_from(kind: TileKind) -> Result<Self, Self::Error> {
      use TileKind::*;
      match kind {
         SolidTopLeft => Ok(Sides::TOP | Sides::LEFT),
         SolidTop | SolidTopFadeLeft | SolidTopFadeRight => Ok(Sides::TOP),
         SolidTopRight => Ok(Sides::TOP | Sides::RIGHT),
         SolidRight | SolidRightFadeTop | SolidRightFadeBottom => Ok(Sides::RIGHT),
         SolidBottomRight => Ok(Sides::BOTTOM | Sides::RIGHT),
         SolidBottom | SolidBottomFadeLeft | SolidBottomFadeRight => Ok(Sides::BOTTOM),
         SolidBottomLeft => Ok(Sides::BOTTOM | Sides::LEFT),
         SolidLeft | SolidLeftFadeTop | SolidLeftFadeBottom => Ok(Sides::LEFT),
         SolidVTop => Ok(Sides::LEFT | Sides::TOP | Sides::RIGHT),
         SolidVMiddle => Ok(Sides::LEFT | Sides::RIGHT),
         SolidVBottom => Ok(Sides::LEFT | Sides::BOTTOM | Sides::RIGHT),
         SolidHLeft => Ok(Sides::TOP | Sides::LEFT | Sides::BOTTOM),
         SolidHCenter => Ok(Sides::TOP | Sides::BOTTOM),
         SolidHRight => Ok(Sides::TOP | Sides::RIGHT | Sides::BOTTOM),
         SolidTile => Ok(Sides::ALL),
         Barrier => todo!(),
         _ => Err(NotAxisAligned),
      }
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
   mesh: Option<Mesh>,
   colliders: Vec<ColliderHandle>,
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
         mesh: None,
         colliders: Vec::new(),
      }
   }

   /// Checks whether the chunk is empty (all tiles in it are [`TileKind::Empty`]).
   pub fn is_empty(&self, tileset: &Tileset) -> bool {
      self.tiles.iter().all(|&tile_id| tileset.kind(tile_id) == TileKind::Empty)
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

/// An in-game map.
pub struct Map {
   pub tileset: Tileset,
   pub layers: Vec<Layer>,
}

impl Map {
   /// Returns the map's tile size. Note that this is not the actual size things are rendered and
   /// simulated at, but rather the size of tiles that should be used in the Tiled map.
   pub fn tile_size() -> Vec2 {
      vector(16.0, 16.0)
   }

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
         layers: Self::load_layers(map.layers, world, physics, &tileset),
         tileset,
      })
   }

   /// Loads all layers from the given Vec.
   fn load_layers(
      layers: Vec<tiled::Layer>,
      world: &mut World,
      physics: &mut Physics,
      tileset: &Tileset,
   ) -> Vec<Layer> {
      layers.into_iter().map(|layer| Self::load_layer(layer, world, physics, tileset)).collect()
   }

   /// Loads a single tiled layer into an actual layer.
   fn load_layer(
      data: tiled::Layer,
      world: &mut World,
      physics: &mut Physics,
      tileset: &Tileset,
   ) -> Layer {
      match data.kind {
         tiled::LayerKind::Tile { chunks } => Self::create_tile_layer(chunks, tileset),
         tiled::LayerKind::Object { objects } => Self::create_object_layer(objects, world, physics),
      }
   }

   /// Creates a new tile layer from a list of chunks.
   fn create_tile_layer(in_chunks: Vec<tiled::Chunk>, tileset: &Tileset) -> Layer {
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
         if !chunk.is_empty(tileset) {
            chunks.insert(chunk_position, chunk);
         }
      }

      Layer::Tile { chunks }
   }
}
