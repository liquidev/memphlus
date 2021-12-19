//! Global storage for game-wide resources.

use std::any::{Any, TypeId};
use std::collections::HashMap;

/// A resource map.
pub struct Resources {
   map: HashMap<TypeId, Box<dyn Any>>,
}

impl Resources {
   /// Creates a new resource map.
   pub fn new() -> Self {
      Self {
         map: HashMap::new(),
      }
   }

   /// Registers a resource.
   pub fn insert<T>(&mut self, value: T)
   where
      T: Any,
   {
      self.map.insert(value.type_id(), Box::new(value));
   }

   /// Returns the resource of the given type.
   pub fn get<T>(&self) -> Option<&T>
   where
      T: Any,
   {
      self.map.get(&TypeId::of::<T>()).map(|value| value.downcast_ref().unwrap())
   }

   /// Returns a mutable reference to the resource of the given type.
   pub fn get_mut<T>(&mut self) -> Option<&mut T>
   where
      T: Any,
   {
      self.map.get_mut(&TypeId::of::<T>()).map(|value| value.downcast_mut().unwrap())
   }

   /// Executes a fallible function to insert a value into the resource map, if not already
   /// in there.
   pub fn try_get_or_insert<T, E>(&mut self, f: impl FnOnce() -> Result<T, E>) -> Result<&mut T, E>
   where
      T: Any,
   {
      let type_id = TypeId::of::<T>();
      if !self.map.contains_key(&type_id) {
         self.map.insert(type_id, Box::new(f()?));
      }
      Ok(self.get_mut().unwrap())
   }
}
