//! Utilities for working with meshes.

use tetra::graphics::mesh::{IndexBuffer, Mesh, Vertex, VertexBuffer};
use tetra::graphics::Color;
use tetra::Context;

use crate::common::{colored_vertex, Rect};

pub struct MeshBuilder {
   vertices: Vec<Vertex>,
   indices: Vec<u32>,
}

impl MeshBuilder {
   pub fn new() -> Self {
      Self {
         vertices: Vec::new(),
         indices: Vec::new(),
      }
   }

   pub fn raw(&mut self, vertices: &[Vertex], indices: &[u32]) -> &mut Self {
      let first_index = self.vertices.len() as u32;
      self.vertices.extend(vertices.iter());
      self.indices.extend(indices.iter().map(|index| index + first_index));
      self
   }

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

   pub fn build(self, ctx: &mut Context) -> anyhow::Result<Mesh> {
      let mut mesh = Mesh::indexed(
         VertexBuffer::new(ctx, &self.vertices)?,
         IndexBuffer::new(ctx, &self.indices)?,
      );
      mesh.set_backface_culling(false);
      Ok(mesh)
   }
}
