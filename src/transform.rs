//! A transform stack that can be used to save and restore transform matrices.

use tetra::{graphics, Context};
use vek::{Mat4, Vec2, Vec3};

pub struct TransformStack {
   stack: Vec<Mat4<f32>>,
}

impl TransformStack {
   /// Creates a new transform stack.
   pub fn new() -> Self {
      Self { stack: Vec::new() }
   }

   /// Saves the current transform onto the stack.
   pub fn save(&mut self, ctx: &mut Context) {
      self.stack.push(graphics::get_transform_matrix(ctx));
   }

   /// Restores the transform matrix off the top of the stack and applies it to the context.
   pub fn restore(&mut self, ctx: &mut Context) {
      if let Some(matrix) = self.stack.pop() {
         graphics::set_transform_matrix(ctx, matrix);
      }
   }
}

/// Translates the transform matrix by the given offset.
pub fn translate(ctx: &mut Context, offset: Vec2<f32>) {
   let matrix = graphics::get_transform_matrix(ctx);
   let matrix = matrix * Mat4::<f32>::translation_2d(offset);
   graphics::set_transform_matrix(ctx, matrix);
}

/// Scales the transform matrix by the given offset.
pub fn scale(ctx: &mut Context, scale: Vec2<f32>) {
   let matrix = graphics::get_transform_matrix(ctx);
   let matrix = matrix * Mat4::<f32>::scaling_3d(Vec3::new(scale.x, scale.y, 1.0));
   graphics::set_transform_matrix(ctx, matrix);
}
