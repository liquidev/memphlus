//! Map tile meshes.

use ggez::graphics::{DrawMode, MeshBuilder, Rect};
use glam::Vec2;

use crate::assets::RemappableColors;
use crate::common::{colored_vertex, rect, vector, Axis, ColorOps, RectCorners};

use super::tiles::{Side, Sides};

pub struct TileMeshes;

impl TileMeshes {
   /// The thickness of tiles.
   const THICKNESS: f32 = 0.1;

   fn side_rect(position: Vec2, side: Side) -> Rect {
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
   pub fn build_sides(
      builder: &mut MeshBuilder,
      position: Vec2,
      sides: Sides,
   ) -> anyhow::Result<()> {
      if sides.contains(Sides::TOP) {
         let rect = Self::side_rect(position, Side::Top);
         builder.rectangle(DrawMode::fill(), rect, RemappableColors::FOREGROUND)?;
      }
      if sides.contains(Sides::LEFT) {
         let rect = Self::side_rect(position, Side::Left);
         builder.rectangle(DrawMode::fill(), rect, RemappableColors::FOREGROUND)?;
      }
      if sides.contains(Sides::BOTTOM) {
         let rect = Self::side_rect(position, Side::Bottom);
         builder.rectangle(DrawMode::fill(), rect, RemappableColors::FOREGROUND)?;
      }
      if sides.contains(Sides::RIGHT) {
         let rect = Self::side_rect(position, Side::Right);
         builder.rectangle(DrawMode::fill(), rect, RemappableColors::FOREGROUND)?;
      }
      Ok(())
   }

   /// Builds the mesh for a fading side. The provided set of sides must contain one element.
   pub fn build_fading_side(
      builder: &mut MeshBuilder,
      position: Vec2,
      side: Side,
      (first_opacity, second_opacity): (f32, f32),
   ) -> anyhow::Result<()> {
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
         None,
      )?;
      Ok(())
   }

   /// Builds spikes pointing at the given side.
   pub fn build_spikes(
      builder: &mut MeshBuilder,
      position: Vec2,
      side: Side,
   ) -> anyhow::Result<()> {
      let vertices = [
         vector(-0.5, 0.5),
         vector(-0.25, 0.0),
         vector(0.0, 0.5),
         vector(0.25, 0.0),
         vector(0.5, 0.5),
      ]
      .map(|offset| {
         let [x, y] = offset.to_array();
         let offset = match side {
            Side::Top => vector(x, y),
            Side::Bottom => vector(x, -y),
            Side::Left => vector(y, -x),
            Side::Right => vector(-y, x),
         };
         colored_vertex(position + offset, RemappableColors::ACCENT)
      });
      builder.raw(&vertices, &[0, 1, 2, 2, 3, 4], None)?;
      Ok(())
   }
}
