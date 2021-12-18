//! Common things - math, conversions, etc.

use std::path::{Path, PathBuf};

use tetra::graphics::mesh::Vertex;
use tetra::graphics::{Color, Rectangle};
use tetra::math::Vec2;

/// Creates a 2D vector.
pub fn vector(x: f32, y: f32) -> Vec2<f32> {
   Vec2::new(x, y)
}

pub trait ToVekVec2 {
   fn vek(self) -> Vec2<f32>;
}

impl ToVekVec2 for nalgebra::Vector2<f32> {
   fn vek(self) -> Vec2<f32> {
      Vec2::new(self.x, self.y)
   }
}

pub trait ToNalgebraVector2 {
   fn nalgebra(self) -> nalgebra::Vector2<f32>;
}

impl ToNalgebraVector2 for Vec2<f32> {
   fn nalgebra(self) -> nalgebra::Vector2<f32> {
      nalgebra::vector![self.x, self.y]
   }
}

pub type Rect = Rectangle<f32>;

/// Creates a rectangle from a point and a size.
pub fn rect(point: Vec2<f32>, size: Vec2<f32>) -> Rect {
   Rect::new(point.x, point.y, size.x, size.y)
}

/// Creates a colored vertex with the UV coordinates set to `(0.0, 0.0)`.
pub fn colored_vertex(position: Vec2<f32>, color: Color) -> Vertex {
   Vertex {
      position,
      uv: vector(0.5, 0.5),
      color,
   }
}

/// Functions for returning extra points in a rectangle.
pub trait RectPoints {
   fn center_point(&self) -> Vec2<f32>;
}

impl RectPoints for Rect {
   fn center_point(&self) -> Vec2<f32> {
      vector(self.x + self.width / 2.0, self.y + self.height / 2.0)
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

pub fn asset_path(path: impl AsRef<Path>) -> PathBuf {
   Path::new("assets").join(path)
}

/// Reads an asset file into a `Vec<u8>`.
pub fn load_asset(path: impl AsRef<Path>) -> anyhow::Result<Vec<u8>> {
   let path = asset_path(path);
   println!("loading {:?}", path);
   Ok(std::fs::read(&path)?)
}

/// Reads an asset file into a `String`.
pub fn load_asset_to_string(path: impl AsRef<Path>) -> anyhow::Result<String> {
   let path = asset_path(path);
   println!("loading {:?}", path);
   Ok(std::fs::read_to_string(path)?)
}
