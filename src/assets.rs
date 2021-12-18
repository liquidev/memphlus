//! Asset loading and bundling.

use tetra::graphics::Color;

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
