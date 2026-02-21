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
