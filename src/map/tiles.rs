//! Tiles and tile layers.
use std::str::FromStr;

/// A map tile.
use bitflags::bitflags;
use serde::de::IntoDeserializer;
use serde::{Deserialize, Serialize};

use crate::common::Axis;

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
