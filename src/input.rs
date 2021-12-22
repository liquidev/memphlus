//! Control scheme-independent input handling.

use std::collections::HashMap;
use std::hash::Hash;

use tetra::input::{self, Key};
use tetra::Context;
use vek::Vec2;

use crate::common::vector;

/// An abstract button that abstracts away an input action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Button {
   Up,
   Down,
   Left,
   Right,
   Jump,
}

/// Layer for handling button mappings.
pub struct Input {
   rev_key_bindings: HashMap<Button, Vec<Key>>,
   joystick: Vec2<f32>,
}

impl Input {
   pub fn new() -> Self {
      let mut input = Self {
         rev_key_bindings: HashMap::new(),
         joystick: vector(0.0, 0.0),
      };

      input.key_binding(Key::W, Button::Up);
      input.key_binding(Key::S, Button::Down);
      input.key_binding(Key::A, Button::Left);
      input.key_binding(Key::D, Button::Right);
      input.key_binding(Key::Space, Button::Jump);

      input.key_binding(Key::Up, Button::Up);
      input.key_binding(Key::Down, Button::Down);
      input.key_binding(Key::Left, Button::Left);
      input.key_binding(Key::Right, Button::Right);
      input.key_binding(Key::X, Button::Jump);

      input
   }

   fn key_binding(&mut self, key: Key, button: Button) {
      self.rev_key_bindings.entry(button).or_default().push(key);
   }

   fn iter_key_bindings(&self, button: Button) -> impl Iterator<Item = Key> + '_ {
      if let Some(bindings) = self.rev_key_bindings.get(&button) {
         Some(bindings.iter().copied())
      } else {
         None
      }
      .into_iter()
      .flatten()
   }

   pub fn button_down(&self, ctx: &Context, button: Button) -> bool {
      self.iter_key_bindings(button).any(|key| input::is_key_down(ctx, key))
   }

   pub fn button_just_pressed(&self, ctx: &Context, button: Button) -> bool {
      self.iter_key_bindings(button).any(|key| input::is_key_pressed(ctx, key))
   }

   /// Returns the joystick direction.
   pub fn joystick(&self) -> Vec2<f32> {
      self.joystick
   }

   /// Updates the input state.
   pub fn tick(&mut self, ctx: &Context) {
      self.joystick = vector(0.0, 0.0);
      if self.button_down(ctx, Button::Left) {
         self.joystick += vector(-1.0, 0.0);
      }
      if self.button_down(ctx, Button::Right) {
         self.joystick += vector(1.0, 0.0);
      }
      if self.button_down(ctx, Button::Up) {
         self.joystick += vector(0.0, -1.0);
      }
      if self.button_down(ctx, Button::Down) {
         self.joystick += vector(0.0, 1.0);
      }
      self.joystick = self.joystick.try_normalized().unwrap_or(vector(0.0, 0.0));
   }
}
