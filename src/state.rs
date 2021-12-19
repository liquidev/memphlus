//! Handling of game states.

// Inside the game loop, states are stored in a `Box<dyn GameState>`. We use a `Box` because we
// need to use `dyn`, as we don't know the concrete type at runtime.
// Using a `Box` also has the nice side effect of almost costless moving, as internally it's
// just a pointer. That's way cheaper than having to copy all the state around!

use std::any::Any;

use tetra::Context;

use crate::input::Input;
use crate::resources::Resources;

/// A game state.
pub trait GameState: Any {
   /// Updates physics and processes input.
   fn update(
      &mut self,
      ctx: &mut Context,
      resources: &mut Resources,
      input: &Input,
   ) -> anyhow::Result<()>;

   /// Draws a single frame of animation.
   fn draw(&mut self, ctx: &mut Context, resources: &mut Resources) -> anyhow::Result<()>;

   /// Called when the window is resized.
   fn resize(&mut self, _ctx: &mut Context, _width: u32, _height: u32) -> anyhow::Result<()> {
      Ok(())
   }

   /// Returns the next state to switch to after this one.
   fn next_state(self: Box<Self>) -> anyhow::Result<Box<dyn GameState>>;
}
