use crate::{Keybindings, Pointer, WidgetId, World};
use crossterm::event::{KeyEvent, MouseEvent, MouseEventKind};

#[derive(Debug)]
pub enum Event {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Tick,
}

impl Event {
    /// Handles the event by dispatching to the appropriate handler.
    ///
    /// # Panics
    ///
    /// Panics if the `Keybindings` resource is not present in the world.
    pub fn handle(self, world: &mut World, ids: &[WidgetId]) {
        match self {
            Event::Key(key) => {
                let binding = (&key).into();
                let keybindings = world.remove::<Keybindings>().unwrap();
                keybindings.handle(&binding, world, ids);
                world.insert(keybindings);
            }
            Event::Mouse(mouse) => {
                let x = mouse.column;
                let y = mouse.row;

                match mouse.kind {
                    MouseEventKind::Down(_) => {
                        let hit = world.get::<Pointer>().hit_test(x, y);
                        world.get_mut::<Pointer>().set_active(hit);

                        if let Some(widget_id) = hit {
                            let handler = world
                                .get::<Pointer>()
                                .get_handler_for(widget_id, mouse.kind);
                            if let Some(f) = handler {
                                f(world, x, y);
                            }
                        }
                    }
                    MouseEventKind::Drag(_) => {
                        // Drag events go to the widget that received mouse down
                        let active = world.get::<Pointer>().active();
                        if let Some(widget_id) = active {
                            let handler = world
                                .get::<Pointer>()
                                .get_handler_for(widget_id, mouse.kind);
                            if let Some(f) = handler {
                                f(world, x, y);
                            }
                        }
                    }
                    MouseEventKind::Up(_) => {
                        // Up events go to the widget that received mouse down
                        let active = world.get::<Pointer>().active();
                        if let Some(widget_id) = active {
                            let handler = world
                                .get::<Pointer>()
                                .get_handler_for(widget_id, mouse.kind);
                            if let Some(f) = handler {
                                f(world, x, y);
                            }
                        }
                        // Clear active widget on mouse up
                        world.get_mut::<Pointer>().set_active(None);
                    }
                    _ => {}
                }
            }
            Event::Tick => {}
        }
    }
}
