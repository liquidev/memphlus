//! Asset loading and bundling.

use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

use serde::de::IntoDeserializer;
use serde::Deserialize;
use tetra::graphics::text::{Font, Text, VectorFontBuilder};
use tetra::graphics::{Color, DrawParams, FilterMode, Texture};
use tetra::Context;

use crate::common::{asset_path, vector};
use crate::resources::Resources;
use crate::tiled::TextHAlign;
use crate::transform::TransformStack;

/// A namespace for colors that are remappable to various colors in the palette.
pub struct RemappableColors;

impl RemappableColors {
   pub const BACKGROUND: Color = Color {
      r: 0.0,
      g: 0.0,
      b: 0.0,
      a: 1.0,
   };
   pub const FOREGROUND: Color = Color {
      r: 1.0,
      g: 0.0,
      b: 0.0,
      a: 1.0,
   };
   pub const ACCENT: Color = Color {
      r: 0.0,
      g: 1.0,
      b: 0.0,
      a: 1.0,
   };
}

/// A texture resource consisting of a single white pixel.
pub struct WhiteTexture(pub Texture);

impl WhiteTexture {
   pub fn insert_to(ctx: &mut Context, resources: &mut Resources) -> anyhow::Result<()> {
      let texture = Texture::from_rgba(ctx, 1, 1, &[255, 255, 255, 255])?;
      Ok(resources.insert::<WhiteTexture>(WhiteTexture(texture)))
   }
}

/// A storage for a single size of a font.
pub struct FontSize {
   size: f32,
   font: Font,
   text: RefCell<Text>,
}

impl FontSize {
   const SCALE: f32 = 6.0;

   /// Draws text to the screen, aligned according to the provided alignment and width.
   ///
   /// This is an internal function that does not perform any line splitting.
   fn draw_aligned_line(
      &self,
      ctx: &mut Context,
      line: &str,
      alignment: TextHAlign,
      alignment_width: f32,
      params: DrawParams,
   ) {
      let mut text_object = self.text.borrow_mut();
      text_object.set_content(line);
      let bounds = text_object.get_bounds(ctx).unwrap();
      let text_width = (bounds.width - bounds.x) / Self::SCALE;
      let x = match alignment {
         TextHAlign::Left => 0.0,
         TextHAlign::Center => alignment_width / 2.0 - text_width / 2.0,
         TextHAlign::Right => alignment_width - text_width,
      };
      let params = DrawParams {
         position: params.position + vector(x, 0.0),
         scale: params.scale * vector(Self::SCALE, Self::SCALE).recip(),
         ..params
      };
      text_object.draw(ctx, params);
   }

   /// Draws text to the screen, aligned according to the provided alignment and width.
   pub fn draw_aligned(
      &self,
      ctx: &mut Context,
      text: &str,
      alignment: TextHAlign,
      alignment_width: f32,
      params: DrawParams,
   ) {
      const LINE_HEIGHT: f32 = 1.2;
      let line_height = self.size * LINE_HEIGHT;
      let mut y = 0.0;
      for line in text.split('\n') {
         let line_params = DrawParams {
            position: params.position + vector(0.0, y),
            ..params
         };
         self.draw_aligned_line(ctx, line, alignment, alignment_width, line_params);
         y += line_height;
      }
   }
}

/// A store for fonts of different sizes.
pub struct FontSizes {
   builder: VectorFontBuilder,
   sizes: HashMap<u32, FontSize>,
}

impl FontSizes {
   /// Loads the font with the given size, if not already loaded.
   pub fn load(&mut self, ctx: &mut Context, size: u32) -> anyhow::Result<&FontSize> {
      if !self.sizes.contains_key(&size) {
         let mut font = self.builder.with_size(ctx, size as f32 * FontSize::SCALE)?;
         font.set_filter_mode(ctx, FilterMode::Linear);
         self.sizes.insert(
            size,
            FontSize {
               size: size as f32,
               text: RefCell::new(Text::new("", font.clone())),
               font,
            },
         );
      }
      Ok(self.sizes.get(&size).unwrap())
   }

   /// Returns the font with the given size, if loaded. Otherwise returns `None`.
   pub fn get(&self, size: u32) -> Option<&FontSize> {
      self.sizes.get(&size)
   }
}

/// A resource consisting of all the different fonts used in the game.
pub struct Fonts {
   pub regular: FontSizes,
}

impl Fonts {
   pub fn load_to(resources: &mut Resources) -> anyhow::Result<()> {
      let regular = FontSizes {
         builder: VectorFontBuilder::new(&asset_path("fonts/Lexend.ttf"))?,
         sizes: HashMap::new(),
      };
      let fonts = Fonts { regular };
      resources.insert(fonts);
      Ok(())
   }
}

/// Font families.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
pub enum FontFamily {
   Lexend,
}

impl FromStr for FontFamily {
   type Err = serde::de::value::Error;

   fn from_str(s: &str) -> Result<Self, Self::Err> {
      Self::deserialize(s.into_deserializer())
   }
}

impl Index<FontFamily> for Fonts {
   type Output = FontSizes;

   fn index(&self, index: FontFamily) -> &Self::Output {
      match index {
         FontFamily::Lexend => &self.regular,
      }
   }
}

impl IndexMut<FontFamily> for Fonts {
   fn index_mut(&mut self, index: FontFamily) -> &mut Self::Output {
      match index {
         FontFamily::Lexend => &mut self.regular,
      }
   }
}
