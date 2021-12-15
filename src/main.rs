mod common;
mod map;
mod physics;
mod state;
mod states;
mod tiled;

use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler};
use ggez::{graphics, timer, Context, ContextBuilder};
use state::{DrawArgs, GameState};

/// The Game. All the relevant state and such.
struct Game {
   state: Option<Box<dyn GameState>>,
}

impl Game {
   const UPDATE_RATE: u32 = 60;

   /// Creates a new game.
   pub fn new(ctx: &mut Context) -> anyhow::Result<Game> {
      Ok(Game {
         state: Some(Box::new(states::game::State::new(ctx)?)),
      })
   }
}

impl EventHandler<Error> for Game {
   fn update(&mut self, ctx: &mut Context) -> Result<(), Error> {
      // Switch states, if needed.
      let state = self.state.take().unwrap();
      let state = state.next_state()?;
      self.state = Some(state);

      // Tick physics and input and all that stuff.
      if timer::check_update_time(ctx, Game::UPDATE_RATE) {
         self.state.as_mut().unwrap().update()?;
      }

      Ok(())
   }

   fn draw(&mut self, ctx: &mut Context) -> Result<(), Error> {
      // Calculate the alpha (interpolation factor).
      const DELTA_TIME: f64 = 1.0 / (Game::UPDATE_RATE as f64);
      let alpha = timer::duration_to_f64(timer::remaining_update_time(ctx)) / DELTA_TIME;
      let alpha = alpha as f32;
      // Draw stuff.
      self.state.as_mut().unwrap().draw(DrawArgs { ctx, alpha })?;

      graphics::present(ctx).wrap_error()
   }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
   let (mut ctx, event_loop) = ContextBuilder::new("memphlus", "liquidev")
      .window_setup(WindowSetup::default().title("mem.pHlus"))
      .window_mode(WindowMode {
         width: 1280.0,
         height: 720.0,
         resizable: true,
         ..Default::default()
      })
      .add_resource_path("assets")
      .build()?;

   let game = Game::new(&mut ctx)?;

   event::run(ctx, event_loop, game)
}

// This hackishness is needed because ggez requires that all errors implement std::error::Error,
// but anyhow::Error doesn't. So we create a wrapper type that forwards all the important
// functionality to the actual type.
// Additionally, we define some methods on Result<T, E> such that we can convert it relatively
// seamlessly to a Result<T, Error>. Not perfect, but it works.
// Most logic is handled in separate files that deal only with anyhow::Errors anyways.

/// Wrapper for [`anyhow::Error`] that implements [`std::error::Error`].
#[derive(Debug)]
#[repr(transparent)]
struct Error(anyhow::Error);

impl From<anyhow::Error> for Error {
   fn from(error: anyhow::Error) -> Self {
      Self(error)
   }
}

impl std::fmt::Display for Error {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      self.0.fmt(f)
   }
}

impl std::error::Error for Error {}

trait WrapError<T, E> {
   fn wrap_error(self) -> Result<T, E>;
}

impl<T, E> WrapError<T, Error> for Result<T, E>
where
   E: Into<anyhow::Error>,
{
   fn wrap_error(self) -> Result<T, Error> {
      self.map_err(|error| Error(anyhow::anyhow!(error)))
   }
}
