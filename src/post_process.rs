//! Canvas for applying post-processing effects.

use std::ops::Deref;

use anyhow::Context as AnyhowContext;
use tetra::graphics::{self, Canvas, DrawParams, Shader};
use tetra::Context;

pub struct PostProcess {
   canvas_a: Canvas,
   canvas_b: Canvas,
}

impl PostProcess {
   pub fn new(ctx: &mut Context, width: i32, height: i32, msaa: u8) -> anyhow::Result<Self> {
      Ok(Self {
         canvas_a: Canvas::builder(width, height)
            .samples(msaa)
            .build(ctx)
            .context("could not create canvas A")?,
         canvas_b: Canvas::builder(width, height)
            .samples(msaa)
            .build(ctx)
            .context("could not create canvas B")?,
      })
   }

   /// Binds the post process canvas for drawing.
   pub fn bind(&self, ctx: &mut Context) {
      graphics::set_canvas(ctx, &self.canvas_a);
   }

   /// Unbinds the post process canvas from drawing.
   pub fn unbind(&self, ctx: &mut Context) {
      graphics::reset_canvas(ctx);
   }

   /// Applies the given pixel effect to the canvas.
   pub fn apply(&mut self, ctx: &mut Context, effect: &PixelEffect) {
      graphics::set_canvas(ctx, &self.canvas_b);
      graphics::set_shader(ctx, effect);
      self.canvas_a.draw(ctx, DrawParams::new());
      graphics::reset_shader(ctx);
      graphics::reset_canvas(ctx);
      std::mem::swap(&mut self.canvas_a, &mut self.canvas_b);
   }

   /// Draws the effect canvas to the screen.
   pub fn draw(&self, ctx: &mut Context, params: DrawParams) {
      self.canvas_a.draw(ctx, params);
   }
}

/// A post-processing shader.
pub struct PixelEffect {
   shader: Shader,
}

impl PixelEffect {
   pub fn new(ctx: &mut Context, source: &str) -> anyhow::Result<Self> {
      Ok(Self {
         // Seems like tetra has a bug where this function accepts a generic parameter for
         // some reason.
         shader: Shader::from_fragment_string::<()>(ctx, source)
            .context("could not create pixel effect")?,
      })
   }
}

impl Deref for PixelEffect {
   type Target = Shader;

   fn deref(&self) -> &Self::Target {
      &self.shader
   }
}
