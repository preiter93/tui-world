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

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Context {
    Global,
    Widget(WidgetId),
}

pub struct DisplayInfo {
    pub key: KeyBinding,
    pub context: Context,
    pub name: &'static str,
    pub description: &'static str,
}

struct Binding {
    key: KeyBinding,
    context: Context,
    action: ActionFn,
    name: &'static str,
    description: &'static str,
}

struct CatchAllHandler {
    context: Context,
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
        ctx: Context,
        key: impl Into<KeyBinding>,
        name: &'static str,
        description: &'static str,
        action: impl Fn(&mut World) + Send + Sync + 'static,
    ) {
        self.bindings.push(Binding {
            key: key.into(),
            context: ctx,
            action: Box::new(action),
            name,
            description,
        });
    }

    pub fn catch_all(
        &mut self,
        ctx: Context,
        handler: impl Fn(&mut World, &KeyBinding) + Send + Sync + 'static,
    ) {
        self.catch_all_handlers.push(CatchAllHandler {
            context: ctx,
            handler: Box::new(handler),
        });
    }

    /// Binds multiple keys to the same action.
    pub fn bind_many(
        &mut self,
        ctx: Context,
        keys: impl Into<Keys>,
        name: &'static str,
        description: &'static str,
        action: impl Fn(&mut World) + Send + Sync + Clone + 'static,
    ) {
        for key in keys.into().0 {
            self.bind(ctx, key, name, description, action.clone());
        }
    }

    pub fn handle(&self, key: &KeyBinding, focus: WidgetId, world: &mut World) -> bool {
        // First, try to find a matching keybinding
        for binding in &self.bindings {
            if binding.key != *key {
                continue;
            }
            match binding.context {
                Context::Global => {
                    (binding.action)(world);
                    return true;
                }
                Context::Widget(w) if w == focus => {
                    (binding.action)(world);
                    return true;
                }
                Context::Widget(_) => {}
            }
        }

        // No binding matched, try catch-all handlers
        for handler in &self.catch_all_handlers {
            match handler.context {
                Context::Global => {
                    (handler.handler)(world, key);
                    return true;
                }
                Context::Widget(w) if w == focus => {
                    (handler.handler)(world, key);
                    return true;
                }
                Context::Widget(_) => {}
            }
        }

        false
    }

    /// Removes all keybindings for the given context.
    pub fn unbind(&mut self, ctx: Context) {
        self.bindings.retain(|b| b.context != ctx);
    }

    #[must_use]
    pub fn display_for(&self, focus: WidgetId) -> Vec<DisplayInfo> {
        self.bindings
            .iter()
            .filter(|b| match b.context {
                Context::Global => true,
                Context::Widget(w) => w == focus,
            })
            .map(|b| DisplayInfo {
                key: b.key,
                context: b.context,
                name: b.name,
                description: b.description,
            })
            .collect()
    }

    #[must_use]
    pub fn display_all(&self) -> Vec<DisplayInfo> {
        self.bindings
            .iter()
            .map(|b| DisplayInfo {
                key: b.key,
                context: b.context,
                name: b.name,
                description: b.description,
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
