//! The state in which you play the game.

use ggez::{graphics, Context};
use glam::Vec2;
use hecs::World;

use crate::assets::RemappableColors;
use crate::common::{read_file_to_string, vector, Transform};
use crate::entities;
use crate::input::Input;
use crate::map::Map;
use crate::physics::Physics;
use crate::state::{DrawArgs, GameState};

/// The state.
pub struct State {
   world: World,
   physics: Physics,
   map: Map,
}

impl State {
   pub fn new(ctx: &mut Context) -> anyhow::Result<Self> {
      let mut world = World::new();
      let mut physics = Physics::new(Vec2::new(0.0, 40.0));
      let map = Map::load_into_world_from_json(
         &mut world,
         &mut physics,
         &read_file_to_string(ctx, "/generated/tileset.json")?,
         &read_file_to_string(ctx, "/generated/map.json")?,
      )?;

      Ok(Self {
         world,
         physics,
         map,
      })
   }
}

impl GameState for State {
   fn update(&mut self, input: &Input) -> anyhow::Result<()> {
      entities::tick_systems(&mut self.world, &mut self.physics, &input);
      self.physics.step();
      Ok(())
   }

   fn draw(&mut self, DrawArgs { ctx, alpha }: DrawArgs) -> anyhow::Result<()> {
      graphics::clear(ctx, RemappableColors::BACKGROUND);

      let transform = Transform::new().scale(vector(32.0, 32.0));
      self.map.draw(ctx, transform)?;
      entities::draw_systems(ctx, &mut self.world, transform, alpha)?;

      Ok(())
   }

   fn next_state(self: Box<Self>) -> anyhow::Result<Box<dyn GameState>> {
      Ok(self)
   }
}
