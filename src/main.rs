mod assets;
mod common;
mod entities;
mod input;
mod map;
mod meshes;
mod palette;
mod physics;
mod state;
mod states;
mod tiled;

use anyhow::Context as AnyhowContext;
use input::Input;
use state::GameState;
use tetra::{graphics, Context, ContextBuilder};
use vek::{Mat4, Vec3};

struct Game {
   state: Option<Box<dyn GameState>>,
   input: Input,
}

impl tetra::State<anyhow::Error> for Game {
   fn update(&mut self, ctx: &mut Context) -> anyhow::Result<()> {
      // Switch states, if needed.
      let state = self.state.take().unwrap();
      let state = state.next_state()?;
      self.state = Some(state);

      // Tick physics and input and all that stuff.
      self.state.as_mut().unwrap().update(ctx, &self.input)?;

      Ok(())
   }

   fn draw(&mut self, ctx: &mut Context) -> anyhow::Result<()> {
      graphics::set_transform_matrix(ctx, Mat4::scaling_3d(Vec3::new(32.0, 32.0, 1.0)));
      self.state.as_mut().unwrap().draw(ctx)?;
      Ok(())
   }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
   let mut ctx = ContextBuilder::new("mem.pHlus", 1280, 720)
      .resizable(true)
      .build()
      .context("could not create ggez::Context")?;

   let state = states::game::State::new(&mut ctx)?;
   let state: Option<Box<dyn GameState>> = Some(Box::new(state));
   let input = Input::new();

   ctx.run(|_| Ok(Game { state, input }))?;
   Ok(())
}
