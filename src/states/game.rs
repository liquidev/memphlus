//! The state in which you play the game.

use ggez::Context;

use crate::state::{DrawArgs, GameState};

/// The state.
pub struct State {}

impl State {
   pub fn new(_ctx: &mut Context) -> Self {
      Self {}
   }
}

impl GameState for State {
   fn update(&mut self) -> anyhow::Result<()> {
      Ok(())
   }

   fn draw(&mut self, DrawArgs { .. }: DrawArgs) -> anyhow::Result<()> {
      Ok(())
   }

   fn next_state(self: Box<Self>) -> anyhow::Result<Box<dyn GameState>> {
      Ok(self)
   }
}
