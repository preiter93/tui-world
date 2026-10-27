use crate::{Keybindings, Pointer, WidgetId, World};
use crossterm::event::{KeyEvent, KeyEventKind, MouseEvent};

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
            // Windows also reports key releases, which would fire every binding twice.
            InputEvent::Key(key) if key.kind == KeyEventKind::Release => {}
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

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyModifiers};

    struct Fired(bool);

    struct TestCase {
        name: &'static str,
        given_kind: KeyEventKind,
        expected_fired: bool,
    }

    #[test]
    fn handle_key() {
        let test_cases = [
            TestCase {
                name: "success: press fires binding",
                given_kind: KeyEventKind::Press,
                expected_fired: true,
            },
            TestCase {
                name: "success: repeat fires binding",
                given_kind: KeyEventKind::Repeat,
                expected_fired: true,
            },
            TestCase {
                name: "failure: release does not fire binding",
                given_kind: KeyEventKind::Release,
                expected_fired: false,
            },
        ];

        for test_case in test_cases {
            let id = WidgetId("widget");
            let mut world = World::default();
            world.insert(Fired(false));
            world
                .get_mut::<Keybindings>()
                .bind(id, 'a', "Test", |world| world.get_mut::<Fired>().0 = true);
            let key = KeyEvent::new_with_kind(
                KeyCode::Char('a'),
                KeyModifiers::NONE,
                test_case.given_kind,
            );

            InputEvent::Key(key).handle(&mut world, &[id]);

            assert_eq!(
                world.get::<Fired>().0,
                test_case.expected_fired,
                "{}",
                test_case.name
            );
        }
    }
}
