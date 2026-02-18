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
