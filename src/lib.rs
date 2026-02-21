//! # tui-world
//!
//! A state and event management library for TUIs built with [ratatui](https://github.com/ratatui/ratatui).
//!
//! ## Core Concepts
//!
//! - [`World`] - A type-safe container that holds application state and resources
//! - [`Keybindings`] - Key binding registry with built-in help display
//! - [`Focus`] - Tracks which widget currently has keyboard focus
//! - [`Pointer`] - Tracks widget areas and handles mouse click/drag/up events
//! - [`InputEvent`] - Unified input event handling for keyboard and mouse
//!
//! ## Example
//!
//! ```ignore
//! use tui_world::prelude::*;
//! use crossterm::event::KeyCode;
//!
//! const WIDGET_ID: WidgetId = WidgetId("MyWidget");
//!
//! let mut world = World::default();
//!
//! // Store state in world
//! world.insert(MyAppState::default());
//!
//! // Register keybindings
//! world.get_mut::<Keybindings>().bind(WIDGET_ID, KeyCode::Enter, "Select", |world| {
//!     world.get_mut::<MyAppState>().select();
//! });
//!
//! // Check focus
//! if world.get::<Focus>().is_focused(WIDGET_ID) {
//!     // widget has focus
//! }
//!
//! // Handle key events
//! InputEvent::Key(key).handle(&mut world, &[WIDGET_ID]);
//! ```

mod focus;
mod input_event;
mod keybindings;
mod pointer;
mod world;

pub use focus::Focus;
pub use input_event::InputEvent;
pub use keybindings::{DisplayInfo, KeyBinding, Keybindings, Keys};
pub use pointer::{Area, Pointer};
pub use world::World;

/// A unique identifier for a widget in the application.
///
/// `WidgetId` is used to associate keybindings, focus state, and pointer areas
/// with specific widgets. It wraps a static string slice for efficient comparison
/// and hashing.
///
/// # Example
/// ```ignore
/// const MY_WIDGET: WidgetId = WidgetId("my_widget");
/// ```
#[derive(Debug, PartialOrd, Ord, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct WidgetId(pub &'static str);

pub mod prelude {
    pub use crate::Area;
    pub use crate::DisplayInfo;
    pub use crate::Focus;
    pub use crate::InputEvent;
    pub use crate::KeyBinding;
    pub use crate::Keybindings;
    pub use crate::Pointer;
    pub use crate::WidgetId;
    pub use crate::World;
}
