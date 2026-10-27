use crate::{Keybindings, Pointer, WidgetId, World};
use crossterm::event::{KeyEvent, MouseEvent};

/// An input event that can be handled by the application.
///
/// `InputEvent` wraps the different types of input the application can receive:
/// keyboard events, mouse events, and periodic tick events for animations or updates.
///
/// Use [`handle`](InputEvent::handle) to dispatch the event to the appropriate
/// keybinding or pointer handler.
///
/// # Example
///
/// ```ignore
/// match crossterm::event::read()? {
///     Event::Key(key) => InputEvent::Key(key).handle(&mut world, &active_widgets),
///     Event::Mouse(mouse) => InputEvent::Mouse(mouse).handle(&mut world, &active_widgets),
///     _ => {}
/// }
/// ```
#[derive(Debug)]
pub enum InputEvent {
    /// A keyboard event.
    Key(KeyEvent),
    /// A mouse event (click, drag, scroll, etc.).
    Mouse(MouseEvent),
    /// A periodic tick event for animations or timed updates.
    Tick,
}

impl InputEvent {
    /// Handles the event by dispatching to the appropriate handler.
    /// Only the widgets in `ids` receive key and mouse events.
    ///
    /// # Panics
    ///
    /// Panics if the `Keybindings` resource is not present in the world.
    pub fn handle(self, world: &mut World, ids: &[WidgetId]) {
        match self {
            InputEvent::Key(key) => {
                let binding = (&key).into();
                let keybindings = world.remove::<Keybindings>().unwrap();
                keybindings.handle(&binding, world, ids);
                world.insert(keybindings);
            }
            InputEvent::Mouse(mouse) => Pointer::handle(world, mouse, ids),
            InputEvent::Tick => {}
        }
    }
}
