//! Tweening and animation utilities.

use std::time::{Duration, Instant};

use tetra::Context;

use crate::interpolation::Lerp;

/// A tween for values of type `T`.
pub struct Tween<T>
where
   T: Copy,
{
   start: T,
   end: T,
   start_time: Instant,
   duration: Duration,
   easing: fn(f32) -> f32,
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
   pub fn start(&mut self, start: T, end: T, duration: Duration, easing: fn(f32) -> f32) {
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
}
