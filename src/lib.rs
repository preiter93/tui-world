mod event;
mod focus;
mod keybindings;
mod pointer;
mod world;

pub use event::Event;
pub use focus::Focus;
pub use keybindings::{DisplayInfo, KeyBinding, Keybindings, Keys};
pub use pointer::{Area, Pointer};
pub use world::World;

#[derive(Debug, PartialOrd, Ord, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct WidgetId(pub &'static str);

pub mod prelude {
    pub use crate::Area;
    pub use crate::DisplayInfo;
    pub use crate::Event;
    pub use crate::Focus;
    pub use crate::KeyBinding;
    pub use crate::Keybindings;
    pub use crate::Pointer;
    pub use crate::WidgetId;
    pub use crate::World;
}
