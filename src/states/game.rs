//! The state in which you play the game.

use hecs::World;
use tetra::math::Vec2;
use tetra::{graphics, Context};

use crate::assets::RemappableColors;
use crate::common::{load_asset_to_string, vector};
use crate::input::Input;
use crate::map::Map;
use crate::physics::Physics;
use crate::state::GameState;
use crate::transform::TransformStack;
use crate::{entities, transform};

/// The state.
pub struct State {
   world: World,
   physics: Physics,
   map: Map,

   tstack: TransformStack,
}

impl State {
   pub fn new(_ctx: &mut Context) -> anyhow::Result<Self> {
      let mut world = World::new();
      let mut physics = Physics::new(Vec2::new(0.0, 40.0));
      let map = Map::load_into_world_from_json(
         &mut world,
         &mut physics,
         &load_asset_to_string("generated/tileset.json")?,
         &load_asset_to_string("generated/map.json")?,
      )?;

      Ok(Self {
         world,
         physics,
         map,

         tstack: TransformStack::new(),
      })
   }
}

impl GameState for State {
   fn update(&mut self, ctx: &mut Context, input: &Input) -> anyhow::Result<()> {
      entities::tick_systems(ctx, &mut self.world, &mut self.physics, input);
      self.physics.step();
      Ok(())
   }

   fn draw(&mut self, ctx: &mut Context) -> anyhow::Result<()> {
      graphics::clear(ctx, RemappableColors::BACKGROUND);

      self.tstack.save(ctx);
      transform::scale(ctx, vector(32.0, 32.0));

      self.map.draw(ctx, &mut self.tstack)?;
      entities::draw_systems(ctx, &mut self.world)?;

      self.tstack.restore(ctx);

      Ok(())
   }

   fn next_state(self: Box<Self>) -> anyhow::Result<Box<dyn GameState>> {
      Ok(self)
   }
}
