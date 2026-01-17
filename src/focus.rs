use crate::WidgetId;

pub struct Focus {
    active: WidgetId,
}

impl Focus {
    #[must_use]
    pub fn new(widget: WidgetId) -> Self {
        Self { active: widget }
    }

    pub fn set(&mut self, widget: WidgetId) {
        self.active = widget;
    }

    #[must_use]
    pub fn get(&self) -> WidgetId {
        self.active
    }
}
