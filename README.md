# tui-world

> ⚠️ **Work in Progress**

A state and event management library for TUIs built with [ratatui](https://github.com/ratatui/ratatui).

## Core Concepts

- **World** - A container that holds application state
- **Keybindings** - Key binding registry with built-in help display
- **Focus** - Tracks which widget is active
- **Pointer** - Tracks widget areas and handles mouse click/drag/up events

## Example

```rust
use crossterm::event::KeyCode;
use tui_world::prelude::*;

const MY_WIDGET_ID: WidgetId = WidgetId("my_widget");

// Store state in world
world.insert(MyAppState::default());

// Register keybindings
let kb = world.get_mut::<Keybindings>();

kb.bind(MY_WIDGET_ID, KeyCode::Enter, "Select", |world| {
    world.get_mut::<MyAppState>().select();
});

// Bind multiple keys to the same action
kb.bind_many(MY_WIDGET_ID, keys![KeyCode::Up, 'k'], "Up", |world| {
    world.get_mut::<MyAppState>().move_up();
});

// Check focus
if world.get::<Focus>().is_focused(MY_WIDGET_ID) {
    // widget has focus
}

// Handle mouse clicks (typically in render function)
world.get_mut::<Pointer>().set(MY_WIDGET_ID, area);
world.get_mut::<Pointer>().on_click(MY_WIDGET_ID, |world, x, y| {
    world.get_mut::<Focus>().set(MY_WIDGET_ID);
});

// Handle events with active widget IDs
let active_ids = vec![MY_WIDGET_ID];
Event::Key(key).handle(&mut world, &active_ids);
Event::Mouse(mouse).handle(&mut world, &active_ids);
```

See `examples/todo` for a complete example.

## License

MIT
