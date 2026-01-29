use crate::WidgetId;
use std::collections::HashMap;

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

#[cfg(feature = "ratatui")]
impl From<ratatui::layout::Rect> for Area {
    fn from(rect: ratatui::layout::Rect) -> Self {
        Self::new(rect.x, rect.y, rect.width, rect.height)
    }
}

#[derive(Default)]
pub struct HitMap {
    order: Vec<WidgetId>,
    rects: HashMap<WidgetId, Area>,
}

impl HitMap {
    pub fn set<W: Into<Area>>(&mut self, id: WidgetId, rect: W) {
        if !self.rects.contains_key(&id) {
            self.order.push(id);
        }
        self.rects.insert(id, rect.into());
    }

    #[must_use]
    pub fn get(&self, id: WidgetId) -> Option<&Area> {
        self.rects.get(&id)
    }

    pub fn remove(&mut self, id: WidgetId) {
        self.rects.remove(&id);
        self.order.retain(|&i| i != id);
    }

    #[must_use]
    pub fn hit_test(&self, x: u16, y: u16) -> Option<WidgetId> {
        for &id in self.order.iter().rev() {
            if let Some(rect) = self.rects.get(&id)
                && rect.contains(x, y)
            {
                return Some(id);
            }
        }
        None
    }
}
