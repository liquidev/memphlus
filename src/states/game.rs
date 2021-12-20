//! The state in which you play the game.

use hecs::{Entity, World};
use tetra::graphics::{Color, DrawParams, StencilAction, StencilState, StencilTest, Texture};
use tetra::math::Vec2;
use tetra::{graphics, window, Context};

use crate::assets::RemappableColors;
use crate::common::{
   load_asset, load_asset_to_string, rect, vector, window_size, Rect, RectVectors,
};
use crate::entities::camera::Camera;
use crate::entities::player::Player;
use crate::input::Input;
use crate::map::Map;
use crate::meshes::MeshBuilder;
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

   player: Entity,
}

impl State {
   /// The percentage of padding to leave along the window's sides.
   const WINDOW_PADDING_PERCENTAGE: f32 = 0.1;

   pub fn new(ctx: &mut Context) -> anyhow::Result<Self> {
      let mut world = World::new();
      let mut physics = Physics::new(Vec2::new(0.0, 40.0));
      let map = Map::load_into_world_from_json(
         &mut world,
         &mut physics,
         &load_asset_to_string("generated/tileset.json")?,
         &load_asset_to_string("generated/map.json")?,
      )?;
      let player = world
         .query_mut::<&Player>()
         .into_iter()
         .next()
         .map(|(id, _)| id)
         .ok_or_else(|| anyhow::anyhow!("the map does not have a player"))?;

      Ok(Self {
         world,
         physics,
         map,

         tstack: TransformStack::new(),
         post_process: Self::resize_post_process(ctx)?,

         palettes: Texture::from_file_data(ctx, &load_asset("images/palettes.png")?)?,
         palette_remap: PixelEffect::new(ctx, &load_asset_to_string("shaders/palette_remap.fsh")?)?,

         player,
      })
   }

   /// Creates a new PostProcess with the window's size.
   fn resize_post_process(ctx: &mut Context) -> anyhow::Result<PostProcess> {
      let (width, height) = window::get_size(ctx);
      PostProcess::new(ctx, width, height, 0)
   }

   fn window_padding(ctx: &Context) -> f32 {
      let window_size = window_size(ctx);
      let padding = window_size * Self::WINDOW_PADDING_PERCENTAGE;
      f32::min(padding.x, padding.y)
   }

   /// Applies the camera transform to the graphics context and returns the screen-space rectangle
   /// the camera is viewing.
   fn apply_camera_transform(&mut self, ctx: &mut Context) -> Rect {
      let camera = Camera::get(&mut self.world, self.player).blend(ctx);
      let window_size = window_size(ctx);
      let padded_window_size = window_size - Self::window_padding(ctx);
      let scale = f32::min(
         padded_window_size.x / camera.width,
         padded_window_size.y / camera.height,
      );

      transform::translate(ctx, window_size / 2.0);
      transform::scale(ctx, vector(scale, scale));
      transform::translate(ctx, -camera.size() / 2.0);
      transform::translate(ctx, -camera.position());

      let camera_size = camera.size() * scale;
      let camera_position = window_size / 2.0 - camera_size / 2.0;
      rect(camera_position, camera_size)
   }

   /// Draws the world. Returns the screen-space camera rectangle.
   fn draw_world(&mut self, ctx: &mut Context, resources: &mut Resources) -> anyhow::Result<Rect> {
      graphics::clear(ctx, RemappableColors::BACKGROUND);

      self.tstack.save(ctx);
      let camera_rect = self.apply_camera_transform(ctx);

      self.map.draw(ctx, &mut self.tstack)?;
      entities::draw_systems(ctx, resources, &mut self.world)?;

      self.tstack.restore(ctx);

      Ok(camera_rect)
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
      graphics::clear(ctx, Color::rgb(0.1, 0.1, 0.1));

      self.post_process.bind(ctx);

      let camera_rect = self.draw_world(ctx, resources)?;

      self.post_process.unbind(ctx);
      self.palette_remap.set_uniform(ctx, "u_palettes", &self.palettes);
      self.post_process.apply(ctx, &self.palette_remap);

      graphics::set_color_mask(ctx, false, false, false, false);
      graphics::set_stencil_state(ctx, StencilState::write(StencilAction::Replace, 255));
      graphics::clear_stencil(ctx, 0);
      MeshBuilder::new()
         .rounded_rectangle(camera_rect, Self::window_padding(ctx) * 0.25, Color::WHITE)
         .build(ctx)?
         .draw(ctx, DrawParams::new().color(Color::RED));
      graphics::set_color_mask(ctx, true, true, true, true);

      graphics::set_stencil_state(ctx, StencilState::read(StencilTest::EqualTo, 255));
      self.post_process.draw(ctx, DrawParams::new());
      graphics::set_stencil_state(ctx, StencilState::disabled());

      Ok(())
   }

   fn resize(&mut self, ctx: &mut Context, _width: i32, _height: i32) -> anyhow::Result<()> {
      self.post_process = Self::resize_post_process(ctx)?;
      Ok(())
   }

   fn next_state(self: Box<Self>) -> anyhow::Result<Box<dyn GameState>> {
      Ok(self)
   }
}
