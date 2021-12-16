//! Common things - math, conversions, etc.

use std::io::Read;
use std::path::Path;

use ggez::graphics::{Color, Rect, Vertex};
use ggez::{filesystem, Context};
use glam::{Affine3A, Mat4, Vec2, Vec3};
use mint::{ColumnMatrix4, IntoMint};

/// Converts linear algebra values around.
pub fn mint<T, U>(value: T) -> U
where
   T: IntoMint,
   T::MintType: Into<U>,
{
   value.into().into()
}

/// Creates a 2D vector.
pub fn vector(x: f32, y: f32) -> Vec2 {
   Vec2::new(x, y)
}

/// Creates a rectangle from a point and a size.
pub fn rect(point: Vec2, size: Vec2) -> Rect {
   Rect::new(point.x, point.y, size.x, size.y)
}

/// Creates a colored vertex with the UV coordinates set to `(0.0, 0.0)`.
pub fn colored_vertex(position: Vec2, color: Color) -> Vertex {
   Vertex {
      pos: position.to_array(),
      uv: [0.0, 0.0],
      color: [color.r, color.g, color.b, color.a],
   }
}

/// Functions for returning the individual corners of a rectangle.
pub trait RectCorners {
   fn top_left(&self) -> Vec2;
   fn top_right(&self) -> Vec2;
   fn bottom_right(&self) -> Vec2;
   fn bottom_left(&self) -> Vec2;
}

impl RectCorners for Rect {
   fn top_left(&self) -> Vec2 {
      vector(self.left(), self.top())
   }

   fn top_right(&self) -> Vec2 {
      vector(self.right(), self.top())
   }

   fn bottom_right(&self) -> Vec2 {
      vector(self.right(), self.bottom())
   }

   fn bottom_left(&self) -> Vec2 {
      vector(self.left(), self.bottom())
   }
}

/// Extra color operations.
pub trait ColorOps {
   fn with_alpha(&self, a: f32) -> Self;
}

impl ColorOps for Color {
   fn with_alpha(&self, a: f32) -> Self {
      Self { a, ..*self }
   }
}

/// An enumeration over the X and Y axes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
   X,
   Y,
}

/// A stacked transformation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform(Mat4);

impl Transform {
   /// Creates the identity transform.
   pub fn new() -> Self {
      Self(Mat4::IDENTITY)
   }

   /// Returns the transform, translated by the given vector.
   pub fn translate(&self, v: Vec2) -> Self {
      Self(self.0 * Mat4::from_translation(Vec3::new(v.x, v.y, 0.0)))
   }

   /// Returns the transform, scaled by the given vector.
   pub fn scale(&self, v: Vec2) -> Self {
      Self(self.0 * Mat4::from_scale(Vec3::new(v.x, v.y, 1.0)))
   }
}

/// Conversion from The Easy Transform to Le Hardé Transform.
impl From<Transform> for ColumnMatrix4<f32> {
   fn from(transform: Transform) -> Self {
      Mat4::from(transform.0).into()
   }
}

/// Reads a file into a `Vec<u8>`.
pub fn read_file(ctx: &mut Context, path: impl AsRef<Path>) -> anyhow::Result<Vec<u8>> {
   let mut bytes = Vec::new();
   let mut file = filesystem::open(ctx, path)?;
   file.read_to_end(&mut bytes)?;
   Ok(bytes)
}

/// Reads a file into a `String`.
pub fn read_file_to_string(ctx: &mut Context, path: impl AsRef<Path>) -> anyhow::Result<String> {
   Ok(std::str::from_utf8(&read_file(ctx, path)?)?.to_owned())
}
