mod assets;
mod common;
mod entities;
mod input;
mod map;
mod physics;
mod state;
mod states;
mod tiled;

use anyhow::Context as AnyhowContext;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::winit_event::{Event, WindowEvent};
use ggez::event::{self, ControlFlow};
use ggez::{graphics, timer, Context, ContextBuilder};
use input::Input;
use state::{DrawArgs, GameState};

use crate::common::{rect, vector};

pub const UPDATE_RATE: u32 = 60;
pub const TIMESTEP: f64 = 1.0 / (UPDATE_RATE as f64);

fn update(
   ctx: &mut Context,
   state_box: &mut Option<Box<dyn GameState>>,
   input: &mut Input,
) -> Result<(), Error> {
   // Switch states, if needed.
   let state = state_box.take().unwrap();
   let state = state.next_state()?;
   *state_box = Some(state);

   // Tick physics and input and all that stuff.
   while timer::check_update_time(ctx, UPDATE_RATE) {
      state_box.as_mut().unwrap().update(input)?;
      input.finish_frame();
   }

   Ok(())
}

fn draw(ctx: &mut Context, state: &mut dyn GameState) -> Result<(), Error> {
   // Resize the window.
   let (window_width, window_height) = graphics::size(ctx);
   graphics::set_screen_coordinates(
      ctx,
      rect(vector(0.0, 0.0), vector(window_width, window_height)),
   )
   .wrap_error()?;

   // Calculate the alpha (interpolation factor).
   let alpha = timer::duration_to_f64(timer::remaining_update_time(ctx)) / TIMESTEP;
   let alpha = alpha as f32;
   // Draw stuff.
   state.draw(DrawArgs { ctx, alpha })?;

   graphics::present(ctx).wrap_error()
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
      .build()
      .context("could not create ggez::Context")?;

   let state = states::game::State::new(&mut ctx)?;
   let mut state: Option<Box<dyn GameState>> = Some(Box::new(state));
   let mut input = Input::new();

   event_loop.run(move |mut event, _, control_flow| {
      *control_flow = ControlFlow::Poll;

      event::process_event(&mut ctx, &mut event);
      match event {
         Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            event => input.process_event(event),
         },
         Event::MainEventsCleared => {
            ctx.timer_context.tick();

            let result = update(&mut ctx, &mut state, &mut input)
               .and_then(|_| draw(&mut ctx, state.as_deref_mut().unwrap()));
            if let Err(error) = result {
               *control_flow = ControlFlow::Exit;
               eprintln!("{:?}", error.0);
            }
         }
         _ => (),
      }
   })
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
