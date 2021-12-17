//! Input handling layer. An abstraction over keyboards, keyboard layouts, and controllers.

use std::collections::HashMap;

use ggez::event::winit_event::{ElementState, KeyboardInput, WindowEvent};
use ggez::event::KeyCode;

/// An input button bound to an action.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Button {
   Left,
   Right,
   Jump,
}

pub struct Input {
   buttons_just_pressed: [bool; Self::MAX_ACTIONS],
   buttons_just_released: [bool; Self::MAX_ACTIONS],
   buttons_down: [bool; Self::MAX_ACTIONS],

   keycode_mappings: HashMap<KeyCode, Button>,
}

impl Input {
   const MAX_ACTIONS: usize = 16;

   /// Creates a new input state struct.
   pub fn new() -> Self {
      Self {
         buttons_just_pressed: [false; Self::MAX_ACTIONS],
         buttons_just_released: [false; Self::MAX_ACTIONS],
         buttons_down: [false; Self::MAX_ACTIONS],
         keycode_mappings: HashMap::from_iter(
            [
               (KeyCode::A, Button::Left),
               (KeyCode::D, Button::Right),
               (KeyCode::Space, Button::Jump),
               (KeyCode::Left, Button::Left),
               (KeyCode::Right, Button::Right),
               (KeyCode::Up, Button::Jump),
            ]
            .into_iter(),
         ),
      }
   }

   /// Returns whether the given button has just been pressed.
   pub fn button_just_pressed(&self, button: Button) -> bool {
      self.buttons_just_pressed[button as usize]
   }

   /// Returns whether the given button has just been released.
   pub fn button_just_released(&self, button: Button) -> bool {
      self.buttons_just_released[button as usize]
   }

   /// Returns whether the given button is being held down.
   pub fn button_down(&self, button: Button) -> bool {
      self.buttons_down[button as usize]
   }

   /// Updates the input state according to the given event.
   pub fn process_event(&mut self, event: WindowEvent) {
      match event {
         WindowEvent::KeyboardInput {
            input:
               KeyboardInput {
                  state,
                  virtual_keycode: Some(keycode),
                  ..
               },
            ..
         } => {
            if let Some(&button) = self.keycode_mappings.get(&keycode) {
               let index = button as usize;
               match state {
                  ElementState::Pressed => {
                     if !self.buttons_down[index] {
                        self.buttons_just_pressed[index] = true;
                     }
                     self.buttons_down[index] = true;
                  }
                  ElementState::Released => {
                     self.buttons_just_released[index] = true;
                     self.buttons_down[index] = false;
                  }
               }
            }
         }
         _ => (),
      }
   }

   /// Finishes processing the current frame's events.
   pub fn finish_frame(&mut self) {
      self.buttons_just_pressed.iter_mut().for_each(|state| *state = false);
      self.buttons_just_released.iter_mut().for_each(|state| *state = false);
   }
}
