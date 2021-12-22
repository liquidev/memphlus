//! Tweening and animation utilities.

use std::time::{Duration, Instant};

use crate::interpolation::Lerp;

/// An easing function.
pub type Easing = fn(f32) -> f32;

/// A tween for values of type `T`.
pub struct Tween<T>
where
   T: Copy,
{
   start: T,
   end: T,
   start_time: Instant,
   duration: Duration,
   easing: Easing,
}

impl<T> Tween<T>
where
   T: Copy,
{
   /// Creates a new tween with the given initial value.
   pub fn new(initial_value: T) -> Self {
      Self {
         start: initial_value,
         end: initial_value,
         start_time: Instant::now(),
         // NOTE: The duration cannot be zero, otherwise there's a division by zero in `lerp`.
         duration: Duration::from_secs(1),
         easing: easings::linear,
      }
   }

   /// Starts animating, with the given start and end values, as well as a duration.
   pub fn start(&mut self, start: T, end: T, duration: Duration, easing: Easing) {
      assert!(
         duration.as_secs_f64() > 0.0,
         "the duration must be longer than zero"
      );
      self.start = start;
      self.end = end;
      self.start_time = Instant::now();
      self.duration = duration;
      self.easing = easing;
   }

   /// Interpolates the tween and returns its value at the current moment.
   ///
   /// Note that because Rust's high precision timing functions are used, two calls to this function
   /// will never yield the same value (unless already at the end of the animation).
   pub fn get(&self) -> T
   where
      T: Lerp<f32>,
   {
      let elapsed = self.start_time.elapsed().as_secs_f64();
      let duration = self.duration.as_secs_f64();
      let factor = (elapsed / duration).clamp(0.0, 1.0);
      let factor = (self.easing)(factor as f32);
      self.start.lerp(self.end, factor)
   }
}

/// Easings for modifying animation curves.
pub mod easings {
   pub fn linear(x: f32) -> f32 {
      x
   }

   pub fn cubic_out(x: f32) -> f32 {
      1.0 - f32::powf(1.0 - x, 3.0)
   }

   pub fn bounce_out(x: f32) -> f32 {
      const C1: f32 = 2.0;
      const C3: f32 = C1 + 1.0;
      1.0 + C3 * f32::powf(x - 1.0, 3.0) + C1 * f32::powf(x - 1.0, 2.0)
   }
}
