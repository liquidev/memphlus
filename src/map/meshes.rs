//! Map tile meshes.

use tetra::math::Vec2;

use crate::assets::RemappableColors;
use crate::common::{colored_vertex, rect, vector, Axis, Rect};
use crate::meshes::MeshBuilder;

use super::tiles::{Corner, Side, Sides};

pub struct TileMeshes;

impl TileMeshes {
   /// The thickness of tiles.
   const THICKNESS: f32 = 0.1;

   fn side_rect(position: Vec2<f32>, side: Side) -> Rect {
      match side {
         Side::Top => rect(position - vector(0.5, 0.5), vector(1.0, Self::THICKNESS)),
         Side::Bottom => rect(
            position + vector(-0.5, 0.5 - Self::THICKNESS),
            vector(1.0, Self::THICKNESS),
         ),
         Side::Left => rect(position - vector(0.5, 0.5), vector(Self::THICKNESS, 1.0)),
         Side::Right => rect(
            position + vector(0.5 - Self::THICKNESS, -0.5),
            vector(Self::THICKNESS, 1.0),
         ),
      }
   }

   /// Builds the mesh for axis-aligned sides.
   pub fn build_sides(builder: &mut MeshBuilder, position: Vec2<f32>, sides: Sides) {
      if sides.contains(Sides::TOP) {
         let rect = Self::side_rect(position, Side::Top);
         builder.rectangle(rect, RemappableColors::FOREGROUND);
      }
      if sides.contains(Sides::LEFT) {
         let rect = Self::side_rect(position, Side::Left);
         builder.rectangle(rect, RemappableColors::FOREGROUND);
      }
      if sides.contains(Sides::BOTTOM) {
         let rect = Self::side_rect(position, Side::Bottom);
         builder.rectangle(rect, RemappableColors::FOREGROUND);
      }
      if sides.contains(Sides::RIGHT) {
         let rect = Self::side_rect(position, Side::Right);
         builder.rectangle(rect, RemappableColors::FOREGROUND);
      }
   }

   /// Builds the mesh for corners.
   pub fn build_corner(builder: &mut MeshBuilder, position: Vec2<f32>, corner: Corner) {
      let size = vector(Self::THICKNESS, Self::THICKNESS);
      let position = position
         + match corner {
            Corner::TopLeft => vector(0.0, 0.0),
            Corner::TopRight => vector(1.0 - size.x, 0.0),
            Corner::BottomRight => vector(1.0 - size.x, 1.0 - size.y),
            Corner::BottomLeft => vector(0.0, 1.0 - size.y),
         }
         - vector(0.5, 0.5);
      let rect = rect(position, size);
      builder.rectangle(rect, RemappableColors::FOREGROUND);
   }

   /// Builds the mesh for a fading side. The provided set of sides must contain one element.
   pub fn build_fading_side(
      builder: &mut MeshBuilder,
      position: Vec2<f32>,
      side: Side,
      (first_opacity, second_opacity): (f32, f32),
   ) {
      let rect = Self::side_rect(position, side);
      let colors = match side.axis() {
         Axis::X => [
            RemappableColors::FOREGROUND.with_alpha(first_opacity), // top left
            RemappableColors::FOREGROUND.with_alpha(second_opacity), // top right
            RemappableColors::FOREGROUND.with_alpha(second_opacity), // bottom right
            RemappableColors::FOREGROUND.with_alpha(first_opacity), // bottom left
         ],
         Axis::Y => [
            RemappableColors::FOREGROUND.with_alpha(first_opacity), // top left
            RemappableColors::FOREGROUND.with_alpha(first_opacity), // top right
            RemappableColors::FOREGROUND.with_alpha(second_opacity), // bottom right
            RemappableColors::FOREGROUND.with_alpha(second_opacity), // bottom left
         ],
      };
      builder.raw(
         &[
            colored_vertex(rect.top_left(), colors[0]),
            colored_vertex(rect.top_right(), colors[1]),
            colored_vertex(rect.bottom_right(), colors[2]),
            colored_vertex(rect.bottom_left(), colors[3]),
         ],
         &[0, 1, 2, 2, 3, 0],
      );
   }

   /// Builds spikes pointing at the given side.
   pub fn build_spikes(builder: &mut MeshBuilder, position: Vec2<f32>, side: Side) {
      let vertices = [
         vector(-0.5, 0.5),
         vector(-0.25, 0.0),
         vector(0.0, 0.5),
         vector(0.25, 0.0),
         vector(0.5, 0.5),
      ]
      .map(|offset| {
         let Vec2 { x, y } = offset;
         let offset = match side {
            Side::Top => vector(x, y),
            Side::Bottom => vector(x, -y),
            Side::Left => vector(y, -x),
            Side::Right => vector(-y, x),
         };
         colored_vertex(position + offset, RemappableColors::ACCENT)
      });
      builder.raw(&vertices, &[0, 1, 2, 2, 3, 4]);
   }
}
