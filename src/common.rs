//! Common things - math, conversions, etc.

use std::io::Read;
use std::path::Path;

use ggez::graphics::Rect;
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
