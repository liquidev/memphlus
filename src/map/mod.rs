//! Map loading, storage, and physics.

mod entities;
mod meshes;
mod rendering;
mod tiles;

use std::collections::HashMap;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

use anyhow::Context;
use hecs::{Entity, World};
use tetra::graphics::mesh::Mesh;
use tetra::math::Vec2;

use crate::common::vector;
use crate::physics::Physics;
use crate::tiled::{self, ObjectId, TileId};

pub use meshes::*;

use self::tiles::TileKind;

/// The tileset of a map.
pub struct Tileset {
   pub kinds: Vec<TileKind>,
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
   Tile { chunks: HashMap<(i32, i32), Chunk> },
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
   pub fn tile_size() -> Vec2<f32> {
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
         layers: Loader {
            objects: HashMap::new(),
         }
         .load_layers(map.layers, world, physics, &tileset),
         tileset,
      })
   }
}

/// Map loading state.
pub(super) struct Loader {
   objects: HashMap<ObjectId, Entity>,
}

impl Loader {
   /// Returns the entity ID of the object with the given ID.
   pub(super) fn entity(&mut self, world: &mut World, object_id: ObjectId) -> Entity {
      if !self.objects.contains_key(&object_id) {
         self.objects.insert(object_id, world.reserve_entity());
      }
      *self.objects.get(&object_id).unwrap()
   }

   /// Loads all layers from the given Vec.
   fn load_layers(
      &mut self,
      layers: Vec<tiled::Layer>,
      world: &mut World,
      physics: &mut Physics,
      tileset: &Tileset,
   ) -> Vec<Layer> {
      layers.into_iter().map(|layer| self.load_layer(layer, world, physics, tileset)).collect()
   }

   /// Loads a single tiled layer into an actual layer.
   fn load_layer(
      &mut self,
      data: tiled::Layer,
      world: &mut World,
      physics: &mut Physics,
      tileset: &Tileset,
   ) -> Layer {
      match data.kind {
         tiled::LayerKind::Tile { chunks } => Self::create_tile_layer(chunks, tileset, physics),
         tiled::LayerKind::Object { objects } => self.create_object_layer(objects, world, physics),
      }
   }
}
