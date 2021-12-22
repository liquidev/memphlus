//! Utilities for frame-by-frame interpolation.

use std::ops::{Add, Mul, Sub};

use tetra::{time, Context};
use vek::Vec2;

/// Linear interpolation between two values of the same type, using a blending coefficient of
/// type `F`.
pub trait Lerp<F>:
   Add<Self, Output = Self> + Mul<F, Output = Self> + Sub<Self, Output = Self> + Copy
{
   fn lerp(&self, rhs: Self, t: F) -> Self {
      *self + (rhs - *self) * t
   }
}

impl Lerp<f32> for f32 {}
impl Lerp<f32> for Vec2<f32> {}

/// A linearly interpolated value.
#[derive(Debug, Clone, Copy)]
pub struct Interpolated<T>
where
   T: Copy,
{
   current: T,
   previous: T,
}

impl<T> Interpolated<T>
where
   T: Copy,
{
   /// Creates a new interpolated value.
   pub fn new(value: T) -> Self {
      Self {
         current: value,
         previous: value,
      }
   }

   /// Resets the interpolator, replacing the previous value with the current one.
   pub fn reset(&mut self) {
      self.previous = self.current;
   }

   /// Replaces the current value with the provided value, without changing the previous value.
   pub fn set(&mut self, new_value: T) {
      self.current = new_value;
   }

   /// Replaces the previous value with the current one, and updates the current value.
   pub fn update(&mut self, new_value: T) {
      self.reset();
      self.set(new_value);
   }

   /// Linearly interpolates between the previous and current value.
   pub fn lerp<F>(&self, coeff: F) -> T
   where
      T: Lerp<F>,
   {
      self.previous.lerp(self.current, coeff)
   }

   /// Like `lerp`, but takes the blending coefficient from the current frame's blend factor.
   pub fn blend(&self, ctx: &Context) -> T
   where
      T: Lerp<f32>,
   {
      self.lerp(time::get_blend_factor(ctx))
   }

   /// Returns the current value.
   pub fn current(&self) -> T {
      self.current
   }
}
