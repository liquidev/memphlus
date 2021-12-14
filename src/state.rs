//! Handling of game states.

// Inside the game loop, states are stored in a `Box<dyn GameState>`. We use a `Box` because we
// need to use `dyn`, as we don't know the concrete type at runtime.
// Using a `Box` also has the nice side effect of almost costless moving, as internally it's
// just a pointer. That's way cheaper than having to copy all the state around!

use std::any::Any;

use ggez::Context;

/// A game state.
pub trait GameState: Any {
   /// Updates physics and processes input.
   fn update(&mut self) -> anyhow::Result<()>;

   /// Draws a single frame of animation.
   fn draw(&mut self, args: DrawArgs) -> anyhow::Result<()>;

   /// Returns the next state to switch to after this one.
   fn next_state(self: Box<Self>) -> anyhow::Result<Box<dyn GameState>>;
}

pub struct DrawArgs<'ctx> {
   pub ctx: &'ctx mut Context,

   /// The linear interpolation factor.
   pub alpha: f32,
}
