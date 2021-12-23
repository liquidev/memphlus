//! Text entity.

use hecs::{Entity, World};
use tetra::graphics::DrawParams;
use tetra::Context;

use crate::assets::{FontFamily, Fonts, RemappableColors};
use crate::common::{vector, Rect, RectVectors};
use crate::map::Map;
use crate::resources::Resources;
use crate::tiled::TextHAlign;
use crate::transform::{self, TransformStack};

use super::{Position, Size};

/// Text component.
pub struct Text {
   font_family: FontFamily,
   h_align: TextHAlign,
   size: u32,
   text: String,
}

impl Text {
   /// Draws text entities.
   pub fn draw(
      ctx: &mut Context,
      tstack: &mut TransformStack,
      resources: &mut Resources,
      world: &mut World,
   ) -> anyhow::Result<()> {
      tstack.save(ctx);
      transform::scale(ctx, Map::tile_size().recip());

      let fonts = resources.get_mut::<Fonts>().unwrap();
      for (_id, (&Position(position), &Size(size), text)) in
         world.query_mut::<(&Position, &Size, &Text)>()
      {
         let font_family = &mut fonts[text.font_family];
         font_family.load(ctx, text.size)?.draw_aligned(
            ctx,
            &text.text,
            text.h_align,
            size.x * Map::tile_size().x,
            DrawParams::new()
               .position(position * Map::tile_size())
               .color(RemappableColors::FOREGROUND),
         );
      }

      tstack.restore(ctx);

      Ok(())
   }

   /// Spawns a text entity into the world.
   pub fn spawn(
      world: &mut World,
      rect: Rect,
      font_family: FontFamily,
      h_align: TextHAlign,
      size: u32,
      text: String,
   ) -> Entity {
      world.spawn((
         Position(rect.position()),
         Size(rect.size()),
         Text {
            font_family,
            h_align,
            size,
            text,
         },
      ))
   }
}
