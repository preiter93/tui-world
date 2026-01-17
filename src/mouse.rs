use crate::WidgetId;
use crate::World;
use std::collections::HashMap;
use std::sync::Arc;

pub type ClickFn = Arc<dyn Fn(&mut World, u16, u16) + Send + Sync>;

pub struct Mouse {
    handlers: HashMap<WidgetId, ClickFn>,
}

impl Default for Mouse {
    fn default() -> Self {
        Self::new()
    }
}

impl Mouse {
    #[must_use]
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn on_click<F>(&mut self, id: WidgetId, handler: F)
    where
        F: Fn(&mut World, u16, u16) + Send + Sync + 'static,
    {
        self.handlers.insert(id, Arc::new(handler));
    }

    #[must_use]
    pub fn get(&self, id: WidgetId) -> Option<ClickFn> {
        self.handlers.get(&id).cloned()
    }

    pub fn remove(&mut self, id: WidgetId) {
        self.handlers.remove(&id);
    }
}
