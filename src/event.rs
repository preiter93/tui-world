use crate::{Focus, Keybindings, Layout, Mouse, World};
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
    pub fn handle(self, world: &mut World) {
        match self {
            Event::Key(key) => {
                let binding = (&key).into();
                let focus = world.get::<Focus>().get();
                let keybindings = world.remove::<Keybindings>().unwrap();
                keybindings.handle(&binding, focus, world);
                world.insert(keybindings);
            }
            Event::Mouse(mouse) => {
                if !matches!(mouse.kind, MouseEventKind::Down(_)) {
                    return;
                }

                let x = mouse.column;
                let y = mouse.row;

                let hit = world.get::<Layout>().hit_test(x, y);

                if let Some(widget_id) = hit {
                    let handler = world.get::<Mouse>().get(widget_id);
                    if let Some(f) = handler {
                        f(world, x, y);
                    }
                }
            }
            Event::Tick => {}
        }
    }
}
