# tui-world

> ⚠️ **Work in Progress**: This library is development.

A state and event management library for TUIs built with [ratatui](https://github.com/ratatui/ratatui).

## Core Concepts

- **World** - A container that holds application state
- **Keybindings** - Key binding registry with built-in help display
- **Focus** - Tracks which widget is active
- **Layout** - Tracks widget areas for mouse hit testing
- **Mouse** - Mouse click handling per widget

## Example

```rust
use tui_world::prelude::*;

// Store state in world
world.insert(MyAppState::default());

// Register keybindings
let kb = world.get_mut::<Keybindings>();
kb.bind(
    Context::Widget(MY_WIDGET_ID),
    KeyBinding::key(KeyCode::Enter),
    "Select",
    "Select the current item",
    |world| {
        world.get_mut::<MyAppState>().select();
    },
);

// Handle events
Event::Key(key).handle(&mut world);
Event::Mouse(mouse).handle(&mut world);
```

See `examples/todo` for a complete example.

## License

MIT
