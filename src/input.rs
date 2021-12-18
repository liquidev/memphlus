//! Control scheme-independent input handling.

use std::collections::HashMap;
use std::hash::Hash;

use tetra::input::{self, Key};
use tetra::Context;

/// An abstract button that abstracts away an input action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Button {
   Left,
   Right,
   Jump,
}

/// Layer for handling button mappings.
pub struct Input {
   rev_key_bindings: HashMap<Button, Vec<Key>>,
}

impl Input {
   pub fn new() -> Self {
      let mut input = Self {
         rev_key_bindings: HashMap::new(),
      };
      input.key_binding(Key::A, Button::Left);
      input.key_binding(Key::D, Button::Right);
      input.key_binding(Key::Space, Button::Jump);
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
}
