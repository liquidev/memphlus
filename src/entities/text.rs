//! Text entity.

use hecs::{Entity, World};

use crate::assets::FontFamily;
use crate::common::{Rect, RectVectors};
use crate::tiled::TextHAlign;

use super::{Position, Size};

/// Text component.
pub struct Text {
   font_family: FontFamily,
   h_align: TextHAlign,
   text: String,
}

impl Text {
   pub fn spawn(
      world: &mut World,
      rect: Rect,
      font_family: FontFamily,
      h_align: TextHAlign,
      text: String,
   ) -> Entity {
      world.spawn((
         Position(rect.position()),
         Size(rect.size()),
         Text {
            font_family,
            h_align,
            text,
         },
      ))
   }
}
