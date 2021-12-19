//! The state in which you play the game.

use hecs::World;
use tetra::graphics::{DrawParams, Texture};
use tetra::math::Vec2;
use tetra::{graphics, window, Context};

use crate::assets::RemappableColors;
use crate::common::{load_asset, load_asset_to_string, vector};
use crate::input::Input;
use crate::map::Map;
use crate::physics::Physics;
use crate::post_process::{PixelEffect, PostProcess};
use crate::resources::Resources;
use crate::state::GameState;
use crate::transform::TransformStack;
use crate::{entities, transform};

/// The state.
pub struct State {
   world: World,
   physics: Physics,
   map: Map,

   tstack: TransformStack,
   post_process: PostProcess,

   palettes: Texture,
   palette_remap: PixelEffect,
}

impl State {
   pub fn new(ctx: &mut Context) -> anyhow::Result<Self> {
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
         post_process: Self::resize_post_process(ctx)?,

         palettes: Texture::from_file_data(ctx, &load_asset("images/palettes.png")?)?,
         palette_remap: PixelEffect::new(ctx, &load_asset_to_string("shaders/palette_remap.fsh")?)?,
      })
   }

   /// Creates a new PostProcess with the window's size.
   fn resize_post_process(ctx: &mut Context) -> anyhow::Result<PostProcess> {
      let (width, height) = window::get_size(ctx);
      PostProcess::new(ctx, width, height, 0)
   }

   fn draw_world(&mut self, ctx: &mut Context, resources: &mut Resources) -> anyhow::Result<()> {
      graphics::clear(ctx, RemappableColors::BACKGROUND);

      self.tstack.save(ctx);
      transform::scale(ctx, vector(32.0, 32.0));

      self.map.draw(ctx, &mut self.tstack)?;
      entities::draw_systems(ctx, resources, &mut self.world)?;

      self.tstack.restore(ctx);

      Ok(())
   }
}

impl GameState for State {
   fn update(
      &mut self,
      ctx: &mut Context,
      _resources: &mut Resources,
      input: &Input,
   ) -> anyhow::Result<()> {
      entities::tick_systems(ctx, &mut self.world, &mut self.physics, input);
      self.physics.step();
      Ok(())
   }

   fn draw(&mut self, ctx: &mut Context, resources: &mut Resources) -> anyhow::Result<()> {
      self.post_process.bind(ctx);
      self.draw_world(ctx, resources)?;
      self.post_process.unbind(ctx);
      self.palette_remap.set_uniform(ctx, "u_palettes", &self.palettes);
      self.post_process.apply(ctx, &self.palette_remap);
      self.post_process.draw(ctx, DrawParams::new());

      Ok(())
   }

   fn next_state(self: Box<Self>) -> anyhow::Result<Box<dyn GameState>> {
      Ok(self)
   }
}
