//! Rendering of the map.

use std::collections::HashMap;

use tetra::graphics::mesh::Mesh;
use tetra::graphics::DrawParams;
use tetra::Context;

use crate::common::vector;
use crate::meshes::MeshBuilder;

use super::tiles::TileKind;
use super::{Chunk, Layer, Map, TileMeshes, Tileset};

impl Map {
   /// Draws the map to the screen.
   pub fn draw(&mut self, ctx: &mut Context) -> anyhow::Result<()> {
      for layer in &mut self.layers {
         layer.draw(&self.tileset, ctx)?;
      }
      Ok(())
   }
}

impl Chunk {
   /// Returns the cached mesh or regenerates the mesh for a chunk.
   fn get_or_generate_mesh(
      &mut self,
      tileset: &Tileset,
      ctx: &mut Context,
   ) -> anyhow::Result<Option<&Mesh>> {
      use super::tiles::TileKind::*;

      if self.mesh.is_none() {
         let mut mesh = MeshBuilder::new();
         let mut has_any_vertices = false;
         for y in 0..Chunk::SIZE {
            for x in 0..Chunk::SIZE {
               let tile_id = self[(x, y)];
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
            return Ok(None);
         }

         self.mesh = Some(mesh.build(ctx)?)
      }
      Ok(self.mesh.as_ref())
   }

   fn draw(&mut self, tileset: &Tileset, ctx: &mut Context) -> anyhow::Result<()> {
      if let Some(mesh) = self.get_or_generate_mesh(tileset, ctx)? {
         mesh.draw(ctx, DrawParams::new())
      }

      Ok(())
   }

   fn fade_opacities(kind: TileKind) -> (f32, f32) {
      use super::tiles::TileKind::*;
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
}

impl Layer {
   fn draw(&mut self, tileset: &Tileset, ctx: &mut Context) -> anyhow::Result<()> {
      match self {
         Layer::Tile { chunks } => Self::draw_chunks(chunks, tileset, ctx),
         Layer::Object => Ok(()),
      }
   }

   fn draw_chunks(
      chunks: &mut HashMap<(u32, u32), Chunk>,
      tileset: &Tileset,
      ctx: &mut Context,
   ) -> anyhow::Result<()> {
      for (&(x, y), chunk) in chunks {
         let offset = vector(x as f32, y as f32) * vector(Chunk::SIZE as f32, Chunk::SIZE as f32);
         // let transform = transform.translate(offset);

         chunk.draw(tileset, ctx)?;
      }
      Ok(())
   }
}
