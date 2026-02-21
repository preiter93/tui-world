# tui-world

[![Crates.io](https://img.shields.io/crates/v/tui-world?logo=rust&style=flat-square&logoColor=E05D44&color=E05D44)](https://crates.io/crates/tui-world)
[![Downloads](https://img.shields.io/crates/d/tui-world?style=flat-square)](https://crates.io/crates/tui-world)
[![Documentation](https://img.shields.io/docsrs/tui-world?style=flat-square&logo=docs.rs)](https://docs.rs/tui-world)
[![CI](https://github.com/preiter93/tui-world/actions/workflows/ci.yml/badge.svg)](https://github.com/preiter93/tui-world/actions/workflows/ci.yml)
[![Dependencies](https://deps.rs/repo/github/preiter93/tui-world/status.svg?style=flat-square)](https://deps.rs/repo/github/preiter93/tui-world)
[![License](https://img.shields.io/crates/l/tui-world?style=flat-square&color=09bd66)](./LICENSE)

A state and event management library for TUIs built with [ratatui](https://github.com/ratatui/ratatui).

## Core Concepts

- **World** - A container that holds application state
- **Keybindings** - Key binding registry with built-in help display
- **Focus** - Tracks which widget is active
- **Pointer** - Tracks widget areas and handles mouse click/drag/up events

## Example

```rust
const GLOBAL_ID: WidgetId = WidgetId("Global");
const WIDGET_ID: WidgetId = WidgetId("MyWidget");

// Store state in world
world.insert(MyAppState::default());

// Register keybindings
let kb = world.get_mut::<Keybindings>();

kb.bind(WIDGET_ID, KeyCode::Enter, "Select", |world| {
    world.get_mut::<MyAppState>().select();
});

// Check focus
if world.get::<Focus>().is_focused(WIDGET_ID) {
    // widget has focus
}

// Register click handlers
world.get_mut::<Pointer>().on_click(WIDGET_ID, |world, area, x, y| {
    let clicked_index = y.saturating_sub(area.y) as usize;
    world.get_mut::<MyAppState>().select(clicked_index);
});

// Register widgets area in render function
world.get_mut::<Pointer>().set(WIDGET_ID, area);

// Handle events with global + focused widget
let mut active = vec![GLOBAL_ID];
if let Some(id) = world.get::<Focus>().id {
    active.push(id);
}
Event::Key(key).handle(&mut world, &active);
Event::Mouse(mouse).handle(&mut world, &active);
```

See `examples/todo` and `examples/help.rs` for complete examples.

## License

MIT
