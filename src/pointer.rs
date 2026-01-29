use crate::WidgetId;
use crate::World;
use std::collections::HashMap;
use std::sync::Arc;

pub type ClickFn = Arc<dyn Fn(&mut World, u16, u16) + Send + Sync>;

#[derive(Debug, Clone, Copy)]
pub struct Area {
    pub x: u16,
    pub y: u16,
    pub width: u16,
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

pub struct Pointer {
    order: Vec<WidgetId>,
    areas: HashMap<WidgetId, Area>,
    handlers: HashMap<WidgetId, ClickFn>,
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

    /// Registers a click handler for a widget.
    pub fn on_click<F>(&mut self, id: WidgetId, handler: F)
    where
        F: Fn(&mut World, u16, u16) + Send + Sync + 'static,
    {
        self.handlers.insert(id, Arc::new(handler));
    }

    /// Returns the click handler for a widget if it exists.
    #[must_use]
    pub fn get_handler(&self, id: WidgetId) -> Option<ClickFn> {
        self.handlers.get(&id).cloned()
    }

    /// Removes a widget's area and handler.
    pub fn remove(&mut self, id: WidgetId) {
        self.areas.remove(&id);
        self.handlers.remove(&id);
        self.order.retain(|&i| i != id);
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
    pub fn hit_test_handler(&self, x: u16, y: u16) -> Option<(WidgetId, ClickFn)> {
        self.hit_test(x, y)
            .and_then(|id| self.handlers.get(&id).cloned().map(|h| (id, h)))
    }
}
