use crate::WidgetId;
use crate::World;
use std::collections::HashMap;
use std::sync::Arc;

pub use crossterm::event::MouseEventKind;
use crossterm::event::{KeyModifiers, MouseEvent};

pub type PointerFn = Arc<dyn Fn(&mut World, &PointerEvent) + Send + Sync>;

/// A mouse event passed to pointer handlers.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct PointerEvent {
    /// The area of the widget that handles the event.
    pub area: Area,
    /// The column of the pointer.
    pub x: u16,
    /// The row of the pointer.
    pub y: u16,
    /// The kind of event, e.g. `Down(MouseButton::Left)`.
    pub kind: MouseEventKind,
    /// The modifiers held during the event.
    pub modifiers: KeyModifiers,
}

/// A rectangular area defined by position and dimensions.
///
/// Used to define clickable regions for widgets in the pointer/mouse handling system.
/// Coordinates are in terminal cell units.
#[derive(Debug, Clone, Copy)]
pub struct Area {
    /// The x-coordinate (column) of the top-left corner.
    pub x: u16,
    /// The y-coordinate (row) of the top-left corner.
    pub y: u16,
    /// The width of the area in cells.
    pub width: u16,
    /// The height of the area in cells.
    pub height: u16,
}

impl Area {
    #[must_use]
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    #[must_use]
    pub fn contains(&self, x: u16, y: u16) -> bool {
        x >= self.x && x < self.x + self.width && y >= self.y && y < self.y + self.height
    }
}

impl From<(u16, u16)> for Area {
    fn from((x, y): (u16, u16)) -> Self {
        Self::new(x, y, 1, 1)
    }
}

#[cfg(feature = "ratatui")]
impl From<ratatui::layout::Rect> for Area {
    fn from(rect: ratatui::layout::Rect) -> Self {
        Self::new(rect.x, rect.y, rect.width, rect.height)
    }
}

#[derive(Default)]
struct Handlers {
    down: Option<PointerFn>,
    drag: Option<PointerFn>,
    up: Option<PointerFn>,
    scroll: Option<PointerFn>,
    moved: Option<PointerFn>,
}

/// Manages mouse/pointer interactions for widgets.
///
/// `Pointer` tracks clickable areas for widgets and dispatches mouse events
/// (down, drag, up, scroll, move) to the appropriate handlers. It maintains a z-order for
/// hit testing, where later-registered widgets appear on top.
///
/// # Example
///
/// ```ignore
/// let mut pointer = Pointer::new();
/// pointer.set(WidgetId("button"), Area::new(10, 5, 20, 3));
/// pointer.on_click(WidgetId("button"), |world, event| {
///     // Handle click at (event.x, event.y) within event.area
/// });
/// ```
pub struct Pointer {
    order: Vec<WidgetId>,
    areas: HashMap<WidgetId, Area>,
    handlers: HashMap<WidgetId, Handlers>,
    /// Tracks which widget received the last mouse down event
    active: Option<WidgetId>,
}

impl Default for Pointer {
    fn default() -> Self {
        Self::new()
    }
}

impl Pointer {
    #[must_use]
    pub fn new() -> Self {
        Self {
            order: Vec::new(),
            areas: HashMap::new(),
            handlers: HashMap::new(),
            active: None,
        }
    }

    /// Sets the clickable area for a widget.
    pub fn set<W: Into<Area>>(&mut self, id: WidgetId, area: W) {
        if !self.areas.contains_key(&id) {
            self.order.push(id);
        }
        self.areas.insert(id, area.into());
    }

    /// Returns the area for a widget if it exists.
    #[must_use]
    pub fn get(&self, id: WidgetId) -> Option<&Area> {
        self.areas.get(&id)
    }

    /// Registers a click (mouse down) handler for a widget.
    pub fn on_click<F>(&mut self, id: WidgetId, handler: F)
    where
        F: Fn(&mut World, &PointerEvent) + Send + Sync + 'static,
    {
        self.handlers.entry(id).or_default().down = Some(Arc::new(handler));
    }

    /// Registers a mouse down handler for a widget.
    pub fn on_down<F>(&mut self, id: WidgetId, handler: F)
    where
        F: Fn(&mut World, &PointerEvent) + Send + Sync + 'static,
    {
        self.on_click(id, handler);
    }

    /// Registers a drag handler for a widget.
    /// This is called when mouse moves while button is held down.
    pub fn on_drag<F>(&mut self, id: WidgetId, handler: F)
    where
        F: Fn(&mut World, &PointerEvent) + Send + Sync + 'static,
    {
        self.handlers.entry(id).or_default().drag = Some(Arc::new(handler));
    }

    /// Registers a mouse up handler for a widget.
    pub fn on_up<F>(&mut self, id: WidgetId, handler: F)
    where
        F: Fn(&mut World, &PointerEvent) + Send + Sync + 'static,
    {
        self.handlers.entry(id).or_default().up = Some(Arc::new(handler));
    }

    /// Registers a scroll handler for the widget under the pointer.
    /// Read `event.kind` to tell the scroll direction.
    pub fn on_scroll<F>(&mut self, id: WidgetId, handler: F)
    where
        F: Fn(&mut World, &PointerEvent) + Send + Sync + 'static,
    {
        self.handlers.entry(id).or_default().scroll = Some(Arc::new(handler));
    }

    /// Registers a handler for pointer movement over a widget while no button is held.
    pub fn on_move<F>(&mut self, id: WidgetId, handler: F)
    where
        F: Fn(&mut World, &PointerEvent) + Send + Sync + 'static,
    {
        self.handlers.entry(id).or_default().moved = Some(Arc::new(handler));
    }

    /// Returns the click/down handler for a widget if it exists.
    #[must_use]
    pub fn get_handler(&self, id: WidgetId) -> Option<PointerFn> {
        self.handlers.get(&id).and_then(|h| h.down.clone())
    }

    /// Removes a widget's area and all handlers.
    pub fn remove(&mut self, id: WidgetId) {
        self.areas.remove(&id);
        self.handlers.remove(&id);
        self.order.retain(|&i| i != id);
        if self.active == Some(id) {
            self.active = None;
        }
    }

    /// Performs a hit test to find which widget is at the given coordinates.
    /// Returns the topmost widget (last in order) that contains the point.
    #[must_use]
    pub fn hit_test(&self, x: u16, y: u16) -> Option<WidgetId> {
        for &id in self.order.iter().rev() {
            if let Some(area) = self.areas.get(&id)
                && area.contains(x, y)
            {
                return Some(id);
            }
        }
        None
    }

    /// Performs a hit test and returns the handler if found.
    #[must_use]
    pub fn hit_test_handler(&self, x: u16, y: u16) -> Option<(WidgetId, PointerFn)> {
        self.hit_test(x, y).and_then(|id| {
            self.handlers
                .get(&id)
                .and_then(|h| h.down.clone())
                .map(|h| (id, h))
        })
    }

    /// Sets the active widget (the one that received mouse down).
    pub fn set_active(&mut self, id: Option<WidgetId>) {
        self.active = id;
    }

    /// Returns the currently active widget.
    #[must_use]
    pub fn active(&self) -> Option<WidgetId> {
        self.active
    }

    /// Gets the handler for a specific event kind and widget.
    #[must_use]
    pub fn get_handler_for(&self, id: WidgetId, kind: MouseEventKind) -> Option<PointerFn> {
        let handlers = self.handlers.get(&id)?;
        match kind {
            MouseEventKind::Down(_) => handlers.down.clone(),
            MouseEventKind::Drag(_) => handlers.drag.clone(),
            MouseEventKind::Up(_) => handlers.up.clone(),
            MouseEventKind::Moved => handlers.moved.clone(),
            MouseEventKind::ScrollDown
            | MouseEventKind::ScrollUp
            | MouseEventKind::ScrollLeft
            | MouseEventKind::ScrollRight => handlers.scroll.clone(),
        }
    }

    /// Dispatches a mouse event to the handler of the widget it belongs to,
    /// if that widget is one of the given active widgets.
    pub fn handle(world: &mut World, mouse: MouseEvent, ids: &[WidgetId]) {
        let x = mouse.column;
        let y = mouse.row;
        let pointer = world.get_mut::<Self>();

        // An inactive widget on top still blocks the widgets below it.
        let hit = pointer.hit_test(x, y).filter(|id| ids.contains(id));

        // Drag and up events go to the widget that received mouse down.
        let target = match mouse.kind {
            MouseEventKind::Down(_) => {
                pointer.set_active(hit);
                hit
            }
            MouseEventKind::Drag(_) | MouseEventKind::Up(_) => pointer.active(),
            MouseEventKind::Moved
            | MouseEventKind::ScrollDown
            | MouseEventKind::ScrollUp
            | MouseEventKind::ScrollLeft
            | MouseEventKind::ScrollRight => hit,
        };

        if let Some(widget_id) = target {
            let pointer = world.get::<Self>();
            let area = pointer.get(widget_id).copied();
            let handler = pointer.get_handler_for(widget_id, mouse.kind);
            if let (Some(area), Some(f)) = (area, handler) {
                let event = PointerEvent {
                    area,
                    x,
                    y,
                    kind: mouse.kind,
                    modifiers: mouse.modifiers,
                };
                f(world, &event);
            }
        }

        if let MouseEventKind::Up(_) = mouse.kind {
            world.get_mut::<Self>().set_active(None);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::MouseButton;

    struct Calls(Vec<String>);

    struct TestCase {
        name: &'static str,
        given_areas: Vec<(WidgetId, Area)>,
        given_ids: Vec<WidgetId>,
        given_events: Vec<MouseEvent>,
        expected_calls: Vec<&'static str>,
    }

    fn mouse(kind: MouseEventKind, x: u16, y: u16) -> MouseEvent {
        MouseEvent {
            kind,
            column: x,
            row: y,
            modifiers: KeyModifiers::NONE,
        }
    }

    #[test]
    fn handle_mouse() {
        let down = MouseEventKind::Down(MouseButton::Left);
        let drag = MouseEventKind::Drag(MouseButton::Left);
        let up = MouseEventKind::Up(MouseButton::Left);

        let test_cases = [
            TestCase {
                name: "success: down on topmost area",
                given_areas: vec![
                    (WidgetId("below"), Area::new(0, 0, 10, 10)),
                    (WidgetId("above"), Area::new(0, 0, 5, 5)),
                ],
                given_ids: vec![WidgetId("below"), WidgetId("above")],
                given_events: vec![mouse(down, 1, 1)],
                expected_calls: vec!["above:Down(Left)"],
            },
            TestCase {
                name: "success: drag and up go to pressed widget",
                given_areas: vec![
                    (WidgetId("left"), Area::new(0, 0, 5, 5)),
                    (WidgetId("right"), Area::new(5, 0, 5, 5)),
                ],
                given_ids: vec![WidgetId("left"), WidgetId("right")],
                given_events: vec![mouse(down, 1, 1), mouse(drag, 6, 1), mouse(up, 6, 1)],
                expected_calls: vec!["left:Down(Left)", "left:Drag(Left)", "left:Up(Left)"],
            },
            TestCase {
                name: "success: right click reaches handler with button",
                given_areas: vec![(WidgetId("left"), Area::new(0, 0, 5, 5))],
                given_ids: vec![WidgetId("left")],
                given_events: vec![mouse(MouseEventKind::Down(MouseButton::Right), 1, 1)],
                expected_calls: vec!["left:Down(Right)"],
            },
            TestCase {
                name: "success: scroll and move go to widget under pointer",
                given_areas: vec![
                    (WidgetId("left"), Area::new(0, 0, 5, 5)),
                    (WidgetId("right"), Area::new(5, 0, 5, 5)),
                ],
                given_ids: vec![WidgetId("left"), WidgetId("right")],
                given_events: vec![
                    mouse(MouseEventKind::ScrollDown, 6, 1),
                    mouse(MouseEventKind::Moved, 1, 1),
                ],
                expected_calls: vec!["right:ScrollDown", "left:Moved"],
            },
            TestCase {
                name: "failure: scroll outside any area",
                given_areas: vec![(WidgetId("left"), Area::new(0, 0, 5, 5))],
                given_ids: vec![WidgetId("left")],
                given_events: vec![mouse(MouseEventKind::ScrollUp, 20, 20)],
                expected_calls: vec![],
            },
            TestCase {
                name: "failure: down on inactive widget",
                given_areas: vec![(WidgetId("left"), Area::new(0, 0, 5, 5))],
                given_ids: vec![],
                given_events: vec![mouse(down, 1, 1), mouse(drag, 2, 1), mouse(up, 2, 1)],
                expected_calls: vec![],
            },
            TestCase {
                name: "failure: inactive widget on top blocks widget below",
                given_areas: vec![
                    (WidgetId("below"), Area::new(0, 0, 10, 10)),
                    (WidgetId("above"), Area::new(0, 0, 5, 5)),
                ],
                given_ids: vec![WidgetId("below")],
                given_events: vec![mouse(down, 1, 1)],
                expected_calls: vec![],
            },
            TestCase {
                name: "failure: down outside any area",
                given_areas: vec![(WidgetId("left"), Area::new(0, 0, 5, 5))],
                given_ids: vec![WidgetId("left")],
                given_events: vec![mouse(down, 20, 20), mouse(drag, 1, 1), mouse(up, 1, 1)],
                expected_calls: vec![],
            },
        ];

        for test_case in test_cases {
            let mut world = World::default();
            world.insert(Calls(Vec::new()));
            for (id, area) in test_case.given_areas {
                let record = move |world: &mut World, event: &PointerEvent| {
                    let call = format!("{}:{:?}", id.0, event.kind);
                    world.get_mut::<Calls>().0.push(call);
                };

                let pointer = world.get_mut::<Pointer>();
                pointer.set(id, area);
                pointer.on_down(id, record);
                pointer.on_drag(id, record);
                pointer.on_up(id, record);
                pointer.on_scroll(id, record);
                pointer.on_move(id, record);
            }

            for event in test_case.given_events {
                Pointer::handle(&mut world, event, &test_case.given_ids);
            }

            assert_eq!(
                world.get::<Calls>().0,
                test_case.expected_calls,
                "{}",
                test_case.name
            );
        }
    }
}
