//! The state in which you play the game.

use std::io::Read;

use ggez::graphics::Color;
use ggez::{filesystem, graphics, Context};
use glam::Vec2;
use hecs::World;

use crate::physics::Physics;
use crate::state::{DrawArgs, GameState};
use crate::tiled;

/// The state.
pub struct State {
   world: World,
   physics: Physics,
}

impl State {
   pub fn new(ctx: &mut Context) -> anyhow::Result<Self> {
      let world = World::new();
      let physics = Physics::new(Vec2::new(0.0, 1.0));

      let mut json = Vec::new();
      let mut file = filesystem::open(ctx, "/generated/map.json")?;
      file.read_to_end(&mut json)?;
      let map = tiled::Map::load_from_json(std::str::from_utf8(&json)?)?;
      println!("{:#?}", map);

      Ok(Self { world, physics })
   }
}

impl GameState for State {
   fn update(&mut self) -> anyhow::Result<()> {
      self.physics.step();
      Ok(())
   }

   fn draw(&mut self, DrawArgs { ctx, .. }: DrawArgs) -> anyhow::Result<()> {
      graphics::clear(ctx, Color::from_rgb(127, 127, 127));
      Ok(())
   }

   fn next_state(self: Box<Self>) -> anyhow::Result<Box<dyn GameState>> {
      Ok(self)
   }
}
