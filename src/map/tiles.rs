//! Tiles and tile layers.
use std::collections::HashMap;
use std::str::FromStr;

/// A map tile.
use bitflags::bitflags;
use rapier2d::prelude::{ColliderBuilder, InteractionGroups};
use serde::de::IntoDeserializer;
use serde::{Deserialize, Serialize};
use vek::Vec2;

use crate::common::{vector, Axis, ToNalgebraVector2};
use crate::physics::{CollisionGroups, Physics};
use crate::tiled;

use super::{Chunk, Layer, Map, Tileset};

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
   SolidCornerTopLeft,
   SolidCornerTopRight,
   SolidCornerBottomRight,
   SolidCornerBottomLeft,
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

   pub fn corner(&self) -> Option<Corner> {
      match self {
         Self::SolidCornerTopLeft => Some(Corner::TopLeft),
         Self::SolidCornerTopRight => Some(Corner::TopRight),
         Self::SolidCornerBottomRight => Some(Corner::BottomRight),
         Self::SolidCornerBottomLeft => Some(Corner::BottomLeft),
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

/// Tile corners.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Corner {
   TopLeft,
   TopRight,
   BottomRight,
   BottomLeft,
}

impl Map {
   /// Creates a new tile layer from a list of chunks.
   pub(super) fn create_tile_layer(
      in_chunks: Vec<tiled::Chunk>,
      tileset: &Tileset,
      physics: &mut Physics,
   ) -> Layer {
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
               let tile_id = chunk_data.data[x + y * Chunk::SIZE].saturating_sub(1);
               chunk[(x, y)] = tile_id;
               let kind = tileset.kind(tile_id);
               let tile_top_left =
                  vector(chunk_data.x as f32, chunk_data.y as f32) + vector(x as f32, y as f32);
               Self::build_tile_extra(kind, physics, tile_top_left + vector(0.5, 0.5));
            }
         }
         if !chunk.is_empty(tileset) {
            chunks.insert(chunk_position, chunk);
         }
      }

      Layer::Tile { chunks }
   }

   /// Adds extra things (such as colliders) to tiles.
   fn build_tile_extra(kind: TileKind, physics: &mut Physics, center: Vec2<f32>) {
      use TileKind::*;

      match kind {
         SpikesUp | SpikesDown | SpikesLeft | SpikesRight => {
            Self::build_spikes_collider(kind.spike_direction().unwrap(), physics, center)
         }
         _ => (),
      }
   }

   /// Adds a collider for spikes pointing at the given side.
   fn build_spikes_collider(side: Side, physics: &mut Physics, center: Vec2<f32>) {
      // Half of the long side of the spikes.
      const LONG_HALF: f32 = 0.75 / 2.0;
      // Half of the short side of the spikes.
      const SHORT_HALF: f32 = 0.25 / 2.0;

      let (position, half_extents) = match side {
         Side::Top => (vector(0.0, 0.5 - SHORT_HALF), vector(LONG_HALF, SHORT_HALF)),
         Side::Bottom => (
            vector(0.0, -0.5 + SHORT_HALF),
            vector(LONG_HALF, SHORT_HALF),
         ),
         Side::Left => (vector(0.5 - SHORT_HALF, 0.0), vector(SHORT_HALF, LONG_HALF)),
         Side::Right => (
            vector(-0.5 + SHORT_HALF, 0.0),
            vector(SHORT_HALF, LONG_HALF),
         ),
      };
      let position = center + position;

      let collider = ColliderBuilder::cuboid(half_extents.x, half_extents.y)
         .translation(position.nalgebra())
         .collision_groups(InteractionGroups::new(
            CollisionGroups::DEADLY,
            CollisionGroups::PLAYER,
         ))
         .build();
      let _collider = physics.colliders.insert(collider);
   }
}
