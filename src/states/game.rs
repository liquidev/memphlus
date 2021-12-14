//! The state in which you play the game.

use std::io::Read;

use ggez::{filesystem, Context};

use crate::state::{DrawArgs, GameState};
use crate::tiled;

/// The state.
pub struct State {}

impl State {
   pub fn new(ctx: &mut Context) -> anyhow::Result<Self> {
      let mut json = Vec::new();
      let mut file = filesystem::open(ctx, "/generated/map.json")?;
      file.read_to_end(&mut json)?;
      let map = tiled::Map::load_from_json(std::str::from_utf8(&json)?)?;
      println!("{:#?}", map);
      Ok(Self {})
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
