use std::any::{Any, TypeId, type_name};
use std::collections::HashMap;

use crate::{Keybindings, Layout, Mouse};

pub struct World {
    resources: HashMap<TypeId, Box<dyn Any>>,
}

impl Default for World {
    fn default() -> Self {
        let mut world = Self::new();
        world.insert(Keybindings::new());
        world.insert(Layout::default());
        world.insert(Mouse::new());
        world
    }
}

impl World {
    #[must_use]
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }

    pub fn insert<R: Any>(&mut self, res: R) {
        self.resources.insert(TypeId::of::<R>(), Box::new(res));
    }

    /// # Panics
    ///
    /// Panics if the resource of type `R` is not found.
    #[must_use]
    pub fn get<R: Any>(&self) -> &R {
        self.try_get()
            .unwrap_or_else(|| panic!("resource not found: {}", type_name::<R>()))
    }

    /// # Panics
    ///
    /// Panics if the resource of type `R` is not found.
    pub fn get_mut<R: Any>(&mut self) -> &mut R {
        self.try_get_mut()
            .unwrap_or_else(|| panic!("resource not found: {}", type_name::<R>()))
    }

    #[must_use]
    pub fn try_get<R: Any>(&self) -> Option<&R> {
        self.resources
            .get(&TypeId::of::<R>())
            .and_then(|b| b.downcast_ref())
    }

    pub fn try_get_mut<R: Any>(&mut self) -> Option<&mut R> {
        self.resources
            .get_mut(&TypeId::of::<R>())
            .and_then(|b| b.downcast_mut())
    }

    #[must_use]
    pub fn exists<R: Any>(&self) -> bool {
        self.resources.contains_key(&TypeId::of::<R>())
    }

    pub fn remove<R: Any>(&mut self) -> Option<R> {
        self.resources
            .remove(&TypeId::of::<R>())
            .and_then(|b| b.downcast().ok())
            .map(|b| *b)
    }
}
