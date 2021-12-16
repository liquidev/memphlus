//! Rendering of the map.

use std::collections::HashMap;

use ggez::graphics::{self, DrawParam, MeshBuilder};
use ggez::Context;

use crate::common::{vector, Transform};

use super::{Chunk, Layer, Map, TileKind, TileMeshes, Tileset};

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

   fn fade_opacities(kind: TileKind) -> (f32, f32) {
      use TileKind::*;
      match kind {
         SolidTopFadeLeft | SolidBottomFadeLeft | SolidLeftFadeTop | SolidRightFadeTop => {
            (0.0, 1.0)
         }
         SolidTopFadeRight | SolidBottomFadeRight | SolidLeftFadeBottom | SolidRightFadeBottom => {
            (1.0, 0.0)
         }
         _ => unreachable!(),
      }
   }

   fn draw_chunk(
      chunk: &Chunk,
      tileset: &Tileset,
      ctx: &mut Context,
      transform: Transform,
   ) -> anyhow::Result<()> {
      use TileKind::*;

      let mut mesh = MeshBuilder::new();
      let mut has_any_vertices = false;
      for y in 0..Chunk::SIZE {
         for x in 0..Chunk::SIZE {
            let tile_id = chunk[(x, y)];
            let tile_position = vector(x as f32, y as f32);
            let center = tile_position + vector(0.5, 0.5);
            let kind = tileset.kind(tile_id);
            let mut block_has_vertices = true;
            match kind {
               SolidTopLeft | SolidTop | SolidTopRight | SolidRight | SolidBottomRight
               | SolidBottom | SolidBottomLeft | SolidLeft | SolidVTop | SolidVMiddle
               | SolidVBottom | SolidHLeft | SolidHCenter | SolidHRight | SolidTile => {
                  TileMeshes::build_sides(&mut mesh, center, kind.try_into().unwrap())?
               }
               SolidTopFadeLeft | SolidBottomFadeLeft | SolidLeftFadeBottom
               | SolidRightFadeBottom | SolidTopFadeRight | SolidBottomFadeRight
               | SolidLeftFadeTop | SolidRightFadeTop => TileMeshes::build_fading_side(
                  &mut mesh,
                  center,
                  kind.side().unwrap(),
                  Self::fade_opacities(kind),
               )?,
               SpikesUp | SpikesRight | SpikesDown | SpikesLeft => {
                  TileMeshes::build_spikes(&mut mesh, center, kind.spike_direction().unwrap())?
               }
               _ => block_has_vertices = false,
            }
            has_any_vertices = has_any_vertices | block_has_vertices;
         }
      }
      if !has_any_vertices {
         return Ok(());
      }

      let mesh = mesh.build(ctx)?;
      graphics::draw(ctx, &mesh, DrawParam::default().transform(transform))?;

      Ok(())
   }
}
