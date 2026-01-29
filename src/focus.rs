use crate::WidgetId;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Focus {
    pub id: Option<WidgetId>,
}

impl Focus {
    /// Creates a new Focus with a specific widget active.
    pub const fn new(id: WidgetId) -> Self {
        Self { id: Some(id) }
    }

    /// Creates a new Focus with nothing active.
    pub const fn none() -> Self {
        Self { id: None }
    }

    /// Updates the focus. Accepts an Option<WidgetId>.
    pub fn set(&mut self, id: impl Into<Option<WidgetId>>) {
        self.id = id.into();
    }

    /// Clears the current focus.
    pub fn clear(&mut self) {
        self.id = None;
    }
}
