use crate::WidgetId;

/// Tracks which widget currently has keyboard focus.
///
/// The `Focus` struct manages a single optional `WidgetId` representing the
/// currently focused widget. Only one widget can have focus at a time, and
/// focus can be cleared entirely.
///
/// # Example
///
/// ```ignore
/// let mut focus = Focus::new(WidgetId("input"));
/// assert!(focus.is_focused(WidgetId("input")));
///
/// focus.set(WidgetId("button"));
/// assert!(focus.is_focused(WidgetId("button")));
///
/// focus.clear();
/// assert!(!focus.is_focused(WidgetId("button")));
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Focus {
    pub id: Option<WidgetId>,
}

impl Focus {
    /// Creates a new Focus with a specific widget active.
    #[must_use]
    pub const fn new(id: WidgetId) -> Self {
        Self { id: Some(id) }
    }

    /// Updates the focus. Accepts an `Option<WidgetId>`.
    pub fn set(&mut self, id: impl Into<Option<WidgetId>>) {
        self.id = id.into();
    }

    /// Clears the current focus.
    pub fn clear(&mut self) {
        self.id = None;
    }

    /// Returns true if the given widget has focus.
    #[must_use]
    pub fn is_focused(&self, id: WidgetId) -> bool {
        self.id == Some(id)
    }
}
