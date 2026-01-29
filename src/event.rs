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
                if !matches!(mouse.kind, MouseEventKind::Down(_)) {
                    return;
                }

                let x = mouse.column;
                let y = mouse.row;

                let handler = world.get::<Pointer>().hit_test_handler(x, y);

                if let Some((_, f)) = handler {
                    f(world, x, y);
                }
            }
            Event::Tick => {}
        }
    }
}
