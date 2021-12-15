//! Rendering of the map.

use std::collections::HashMap;

use ggez::graphics::{self, Color, DrawMode, DrawParam, MeshBuilder};
use ggez::Context;

use crate::common::{rect, vector, Transform};

use super::{Chunk, Layer, Map, TileKind, Tileset};

impl Map {
   /// Draws the map to the screen.
   pub fn draw(&self, ctx: &mut Context, transform: Transform) -> anyhow::Result<()> {
      for layer in &self.layers {
         layer.draw(&self.tileset, ctx, transform.into())?;
      }
      Ok(())
   }
}

impl Layer {
   fn draw(
      &self,
      tileset: &Tileset,
      ctx: &mut Context,
      transform: Transform,
   ) -> anyhow::Result<()> {
      match &self {
         Layer::Tile { chunks } => Self::draw_chunks(chunks, tileset, ctx, transform),
         Layer::Object => Ok(()),
      }
   }

   fn draw_chunks(
      chunks: &HashMap<(u32, u32), Chunk>,
      tileset: &Tileset,
      ctx: &mut Context,
      transform: Transform,
   ) -> anyhow::Result<()> {
      for (&(x, y), chunk) in chunks {
         let offset = vector(x as f32, y as f32) * vector(Chunk::SIZE as f32, Chunk::SIZE as f32);
         let transform = transform.translate(offset);

         Self::draw_chunk(chunk, tileset, ctx, transform)?;
      }
      Ok(())
   }

   fn draw_chunk(
      chunk: &Chunk,
      tileset: &Tileset,
      ctx: &mut Context,
      transform: Transform,
   ) -> anyhow::Result<()> {
      let mut mesh = MeshBuilder::new();
      let mut non_empty = false;
      for y in 0..Chunk::SIZE {
         for x in 0..Chunk::SIZE {
            let tile_id = chunk[(x, y)];
            let tile_position = vector(x as f32, y as f32);
            match tileset.kind(tile_id) {
               TileKind::Empty => (),
               _ => {
                  mesh.rectangle(
                     DrawMode::fill(),
                     rect(tile_position, vector(1.0, 1.0)),
                     Color::BLACK,
                  )?;
                  non_empty = true;
               }
            }
         }
      }
      if !non_empty {
         return Ok(());
      }

      let mesh = mesh.build(ctx)?;
      graphics::draw(ctx, &mesh, DrawParam::default().transform(transform))?;

      Ok(())
   }
}
