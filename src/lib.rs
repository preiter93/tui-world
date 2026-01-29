mod event;
mod focus;
mod keybindings;
mod layout;
mod mouse;
mod world;

pub use event::Event;
pub use focus::Focus;
pub use keybindings::{Context, DisplayInfo, KeyBinding, Keybindings, Keys};
pub use layout::{Area, HitMap};
pub use mouse::Mouse;
pub use world::World;

#[derive(Debug, PartialOrd, Ord, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct WidgetId(pub &'static str);

pub mod prelude {
    pub use crate::Area;
    pub use crate::Context;
    pub use crate::DisplayInfo;
    pub use crate::Event;
    pub use crate::Focus;
    pub use crate::HitMap;
    pub use crate::KeyBinding;
    pub use crate::Keybindings;
    pub use crate::Mouse;
    pub use crate::WidgetId;
    pub use crate::World;
}
