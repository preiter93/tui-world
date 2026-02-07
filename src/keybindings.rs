use crate::{WidgetId, World};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Helper macro to create `Keys`.
///
/// # Example
/// ```ignore
/// kb.bind_many(ctx, keys![KeyCode::Up, 'k'], "Up", "", |world| { });
/// ```
#[macro_export]
macro_rules! keys {
    ($($key:expr),+ $(,)?) => {
        $crate::Keys(vec![$($key.into()),+])
    };
}

pub type ActionFn = Box<dyn Fn(&mut World) + Send + Sync>;
pub type CatchAllFn = Box<dyn Fn(&mut World, &KeyBinding) + Send + Sync>;

#[derive(Debug)]
pub struct DisplayInfo {
    pub key: KeyBinding,
    pub id: WidgetId,
    pub name: &'static str,
}

struct Binding {
    key: KeyBinding,
    id: WidgetId,
    action: ActionFn,
    name: &'static str,
}

struct CatchAllHandler {
    id: WidgetId,
    handler: CatchAllFn,
}

pub struct Keybindings {
    bindings: Vec<Binding>,
    catch_all_handlers: Vec<CatchAllHandler>,
}

impl Default for Keybindings {
    fn default() -> Self {
        Self::new()
    }
}

impl Keybindings {
    #[must_use]
    pub fn new() -> Self {
        Self {
            bindings: Vec::new(),
            catch_all_handlers: Vec::new(),
        }
    }

    pub fn bind(
        &mut self,
        id: WidgetId,
        key: impl Into<KeyBinding>,
        name: &'static str,
        action: impl Fn(&mut World) + Send + Sync + 'static,
    ) {
        let key = key.into();
        self.bindings.retain(|b| !(b.key == key && b.id == id));
        self.bindings.push(Binding {
            id,
            key,
            action: Box::new(action),
            name,
        });
    }

    pub fn catch_all(
        &mut self,
        id: WidgetId,
        handler: impl Fn(&mut World, &KeyBinding) + Send + Sync + 'static,
    ) {
        self.catch_all_handlers.push(CatchAllHandler {
            id,
            handler: Box::new(handler),
        });
    }

    /// Binds multiple keys to the same action.
    pub fn bind_many(
        &mut self,
        id: WidgetId,
        keys: impl Into<Keys>,
        name: &'static str,
        action: impl Fn(&mut World) + Send + Sync + Clone + 'static,
    ) {
        for key in keys.into().0 {
            self.bind(id, key, name, action.clone());
        }
    }

    pub fn handle(&self, key: &KeyBinding, world: &mut World, ids: &[WidgetId]) -> bool {
        for binding in &self.bindings {
            if binding.key != *key {
                continue;
            }

            if ids.contains(&binding.id) {
                (binding.action)(world);
                return true;
            }
        }

        for handler in &self.catch_all_handlers {
            if ids.contains(&handler.id) {
                (handler.handler)(world, key);
                return true;
            }
        }

        false
    }

    /// Removes all keybindings for the given context.
    pub fn unbind(&mut self, id: WidgetId) {
        self.bindings.retain(|b| b.id != id);
    }

    #[must_use]
    pub fn display_for(&self, id: &[WidgetId]) -> Vec<DisplayInfo> {
        self.bindings
            .iter()
            .filter(|b| id.contains(&b.id))
            .map(|b| DisplayInfo {
                key: b.key,
                id: b.id,
                name: b.name,
            })
            .collect()
    }

    #[must_use]
    pub fn display_all(&self) -> Vec<DisplayInfo> {
        self.bindings
            .iter()
            .map(|b| DisplayInfo {
                key: b.key,
                id: b.id,
                name: b.name,
            })
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyBinding {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}

impl KeyBinding {
    #[must_use]
    pub fn new(code: KeyCode, modifiers: KeyModifiers) -> Self {
        Self { code, modifiers }
    }

    #[must_use]
    pub fn key(code: KeyCode) -> Self {
        Self::new(code, KeyModifiers::NONE)
    }

    #[must_use]
    pub fn ctrl(c: char) -> Self {
        Self::new(KeyCode::Char(c), KeyModifiers::CONTROL)
    }

    #[must_use]
    pub fn alt(c: char) -> Self {
        Self::new(KeyCode::Char(c), KeyModifiers::ALT)
    }

    #[must_use]
    pub fn shift(c: char) -> Self {
        Self::new(KeyCode::Char(c), KeyModifiers::SHIFT)
    }

    #[must_use]
    pub fn display(&self) -> String {
        let mut parts = Vec::new();

        if self.modifiers.contains(KeyModifiers::CONTROL) {
            parts.push("Ctrl");
        }
        if self.modifiers.contains(KeyModifiers::ALT) {
            parts.push("Alt");
        }
        if self.modifiers.contains(KeyModifiers::SHIFT) {
            parts.push("Shift");
        }

        let key = match self.code {
            KeyCode::Char(c) => {
                if c == ' ' {
                    return "Space".to_string();
                }
                c.to_string()
            }
            KeyCode::Up => "↑".to_string(),
            KeyCode::Down => "↓".to_string(),
            KeyCode::Left => "←".to_string(),
            KeyCode::Right => "→".to_string(),
            KeyCode::Enter => "Enter".to_string(),
            KeyCode::Esc => "Esc".to_string(),
            KeyCode::Tab => "Tab".to_string(),
            KeyCode::Backspace => "Backspace".to_string(),
            KeyCode::Delete => "Delete".to_string(),
            KeyCode::Home => "Home".to_string(),
            KeyCode::End => "End".to_string(),
            KeyCode::PageUp => "PageUp".to_string(),
            KeyCode::PageDown => "PageDown".to_string(),
            KeyCode::F(n) => format!("F{n}"),
            _ => format!("{:?}", self.code),
        };

        if parts.is_empty() {
            key
        } else {
            parts.push(&key);
            parts.join("+")
        }
    }
}

impl From<&KeyEvent> for KeyBinding {
    fn from(event: &KeyEvent) -> Self {
        Self::new(event.code, event.modifiers)
    }
}

impl From<KeyCode> for KeyBinding {
    fn from(code: KeyCode) -> Self {
        Self::key(code)
    }
}

impl From<char> for KeyBinding {
    fn from(c: char) -> Self {
        Self::key(KeyCode::Char(c))
    }
}

/// A collection of keybindings for use with `bind_many`.
#[derive(Debug, Clone)]
pub struct Keys(pub Vec<KeyBinding>);

impl From<Vec<KeyBinding>> for Keys {
    fn from(keys: Vec<KeyBinding>) -> Self {
        Self(keys)
    }
}

impl std::fmt::Display for KeyBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}
