mod assets;
mod common;
mod entities;
mod input;
mod interpolation;
mod map;
mod meshes;
mod palette;
mod physics;
mod post_process;
mod resources;
mod state;
mod states;
mod tiled;
mod transform;
mod tween;

use anyhow::Context as AnyhowContext;
use common::WhiteTexture;
use input::Input;
use resources::Resources;
use state::GameState;
use tetra::{Context, ContextBuilder, Event};

struct Game {
   state: Option<Box<dyn GameState>>,
   input: Input,
   resources: Resources,
}

impl tetra::State<anyhow::Error> for Game {
   fn update(&mut self, ctx: &mut Context) -> anyhow::Result<()> {
      // Switch states, if needed.
      let state = self.state.take().unwrap();
      let state = state.next_state()?;
      self.state = Some(state);

      // Tick physics and input and all that stuff.
      self.input.tick(ctx);
      self.state.as_mut().unwrap().update(ctx, &mut self.resources, &self.input)?;

      Ok(())
   }

   fn draw(&mut self, ctx: &mut Context) -> anyhow::Result<()> {
      self.state.as_mut().unwrap().draw(ctx, &mut self.resources)?;
      Ok(())
   }

   fn event(&mut self, ctx: &mut Context, event: Event) -> anyhow::Result<()> {
      match event {
         Event::Resized { width, height } => {
            self.state.as_mut().unwrap().resize(ctx, width, height)?;
         }
         _ => (),
      }
      Ok(())
   }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
   let mut ctx = ContextBuilder::new("mem.pHlus", 1280, 720)
      .resizable(true)
      .show_mouse(true)
      .stencil_buffer(true)
      .multisampling(8)
      .build()
      .context("could not create tetra::Context")?;

   let state = states::game::State::new(&mut ctx)?;
   let state: Option<Box<dyn GameState>> = Some(Box::new(state));
   let input = Input::new();
   let mut resources = Resources::new();

   WhiteTexture::insert_to(&mut ctx, &mut resources)?;

   ctx.run(|_| {
      Ok(Game {
         state,
         input,
         resources,
      })
   })?;
   Ok(())
}
