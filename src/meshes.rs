//! Utilities for working with meshes.

use tetra::graphics::mesh::{IndexBuffer, Mesh, Vertex, VertexBuffer};
use tetra::graphics::Color;
use tetra::Context;
use vek::Vec2;

use crate::common::{colored_vertex, rect, vector, Rect, RectVectors};

/// A builder for meshes.
pub struct MeshBuilder {
   vertices: Vec<Vertex>,
   indices: Vec<u32>,
}

impl MeshBuilder {
   /// Creates a new mesh builder.
   pub fn new() -> Self {
      Self {
         vertices: Vec::new(),
         indices: Vec::new(),
      }
   }

   /// Adds raw vertices and indices into the mesh.
   pub fn raw(&mut self, vertices: &[Vertex], indices: &[u32]) -> &mut Self {
      let first_index = self.vertices.len() as u32;
      self.vertices.extend(vertices.iter());
      self.indices.extend(indices.iter().map(|index| index + first_index));
      self
   }

   /// Adds a rectangle into the mesh.
   pub fn rectangle(&mut self, rect: Rect, color: Color) -> &mut Self {
      self.raw(
         &[
            colored_vertex(rect.top_left(), color),
            colored_vertex(rect.top_right(), color),
            colored_vertex(rect.bottom_right(), color),
            colored_vertex(rect.bottom_left(), color),
         ],
         &[0, 1, 2, 2, 3, 0],
      );
      self
   }

   /// Adds an arc into the mesh.
   pub fn arc(
      &mut self,
      center: Vec2<f32>,
      radius: f32,
      start_angle: f32,
      end_angle: f32,
      color: Color,
   ) -> &mut Self {
      let subdivisions = ((end_angle - start_angle).abs() * radius).max(6.0) as u32;
      let center_index = self.vertices.len() as u32;
      let center_vertex = colored_vertex(center, color);
      self.vertices.push(center_vertex);

      let first_rim_index = self.vertices.len() as u32;
      for angle_vector in Rotate::new(start_angle, end_angle, subdivisions) {
         self.vertices.push(colored_vertex(center + angle_vector * radius, color));
      }
      for index in 0..subdivisions - 1 {
         let index = first_rim_index + index;
         self.indices.push(center_index);
         self.indices.push(index);
         self.indices.push(index + 1);
      }

      self
   }

   /// Adds a rounded rectangle into the mesh.
   pub fn rounded_rectangle(&mut self, rectangle: Rect, radius: f32, color: Color) -> &mut Self {
      // Inner rectangle
      self.rectangle(
         rect(
            rectangle.position() + vector(radius, radius),
            rectangle.size() - vector(radius, radius) * 2.0,
         ),
         color,
      );

      // Outer rectangles
      self.rectangle(
         rect(
            rectangle.position() + vector(radius, 0.0),
            vector(rectangle.width - radius * 2.0, radius),
         ),
         color,
      );
      self.rectangle(
         rect(
            vector(
               rectangle.x + radius,
               rectangle.y + rectangle.height - radius,
            ),
            vector(rectangle.width - radius * 2.0, radius),
         ),
         color,
      );
      self.rectangle(
         rect(
            vector(rectangle.x, rectangle.y + radius),
            vector(radius, rectangle.height - radius * 2.0),
         ),
         color,
      );
      self.rectangle(
         rect(
            vector(rectangle.x + rectangle.width - radius, rectangle.y + radius),
            vector(radius, rectangle.height - radius * 2.0),
         ),
         color,
      );

      // Corners
      use std::f32::consts::PI;
      self.arc(
         rectangle.top_left() + vector(radius, radius),
         radius,
         PI * 1.0,
         PI * 1.5,
         color,
      );
      self.arc(
         rectangle.top_right() + vector(-radius, radius),
         radius,
         PI * 1.5,
         PI * 2.0,
         color,
      );
      self.arc(
         rectangle.bottom_right() + vector(-radius, -radius),
         radius,
         PI * 0.0,
         PI * 0.5,
         color,
      );
      self.arc(
         rectangle.bottom_left() + vector(radius, -radius),
         radius,
         PI * 0.5,
         PI * 1.0,
         color,
      );

      self
   }

   pub fn build(&mut self, ctx: &mut Context) -> anyhow::Result<Mesh> {
      let mut mesh = Mesh::indexed(
         VertexBuffer::new(ctx, &self.vertices)?,
         IndexBuffer::new(ctx, &self.indices)?,
      );
      mesh.set_backface_culling(false);
      Ok(mesh)
   }
}

struct Rotate {
   current_vertex: u32,
   vertex_count: u32,
   angle_vector: Vec2<f32>,
   sin: f32,
   cos: f32,
}

impl Rotate {
   fn new(start_angle: f32, end_angle: f32, vertex_count: u32) -> Self {
      let delta_angle = (end_angle - start_angle) / (vertex_count - 1) as f32;
      let cos = delta_angle.cos();
      let sin = delta_angle.sin();
      Self {
         current_vertex: 0,
         vertex_count,
         angle_vector: vector(start_angle.cos(), start_angle.sin()),
         sin,
         cos,
      }
   }
}

impl Iterator for Rotate {
   type Item = Vec2<f32>;

   fn next(&mut self) -> Option<Self::Item> {
      if self.current_vertex < self.vertex_count {
         let angle_vector = self.angle_vector;
         self.angle_vector = vector(
            angle_vector.x * self.cos - angle_vector.y * self.sin,
            angle_vector.x * self.sin + angle_vector.y * self.cos,
         );
         self.current_vertex += 1;
         Some(angle_vector)
      } else {
         None
      }
   }
}
