use crate::{Keybindings, Pointer, WidgetId, World};
use crossterm::event::{KeyEvent, MouseEvent, MouseEventKind};

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
            InputEvent::Mouse(mouse) => {
                let x = mouse.column;
                let y = mouse.row;

                match mouse.kind {
                    MouseEventKind::Down(_) => {
                        let hit = world.get::<Pointer>().hit_test(x, y);
                        world.get_mut::<Pointer>().set_active(hit);

                        if let Some(widget_id) = hit {
                            let pointer = world.get::<Pointer>();
                            let area = pointer.get(widget_id).copied();
                            let handler = pointer.get_handler_for(widget_id, mouse.kind);
                            if let (Some(area), Some(f)) = (area, handler) {
                                f(world, area, x, y);
                            }
                        }
                    }
                    MouseEventKind::Drag(_) => {
                        // Drag events go to the widget that received mouse down
                        let active = world.get::<Pointer>().active();
                        if let Some(widget_id) = active {
                            let pointer = world.get::<Pointer>();
                            let area = pointer.get(widget_id).copied();
                            let handler = pointer.get_handler_for(widget_id, mouse.kind);
                            if let (Some(area), Some(f)) = (area, handler) {
                                f(world, area, x, y);
                            }
                        }
                    }
                    MouseEventKind::Up(_) => {
                        // Up events go to the widget that received mouse down
                        let active = world.get::<Pointer>().active();
                        if let Some(widget_id) = active {
                            let pointer = world.get::<Pointer>();
                            let area = pointer.get(widget_id).copied();
                            let handler = pointer.get_handler_for(widget_id, mouse.kind);
                            if let (Some(area), Some(f)) = (area, handler) {
                                f(world, area, x, y);
                            }
                        }
                        // Clear active widget on mouse up
                        world.get_mut::<Pointer>().set_active(None);
                    }
                    _ => {}
                }
            }
            InputEvent::Tick => {}
        }
    }
}
