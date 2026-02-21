use crate::{WidgetId, World};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Helper macro to create `Keys`.
///
/// # Example
/// ```ignore
/// kb.bind_many(ctx, keys![KeyCode::Up, 'k'], "Up", |world| { });
/// ```
#[macro_export]
macro_rules! keys {
    ($($key:expr),+ $(,)?) => {
        $crate::Keys(vec![$($key.into()),+])
    };
}

pub type ActionFn = Box<dyn Fn(&mut World) + Send + Sync>;
pub type AnyKeyFn = Box<dyn Fn(&mut World, &KeyBinding) + Send + Sync>;

struct Binding {
    key: KeyBinding,
    id: WidgetId,
    action: ActionFn,
    name: &'static str,
}

struct AnyKeyHandler {
    id: WidgetId,
    handler: AnyKeyFn,
}

/// A registry of keyboard shortcuts mapped to actions for different widgets.
///
/// `Keybindings` manages the association between key combinations and their
/// corresponding actions, scoped by widget ID. It supports:
/// - Single key bindings via [`bind`](Keybindings::bind)
/// - Multiple keys for the same action via [`bind_many`](Keybindings::bind_many)
/// - Catch-all handlers via [`bind_any`](Keybindings::bind_any)
///
/// # Example
/// ```ignore
/// let mut kb = Keybindings::new();
/// kb.bind(WidgetId("list"), 'j', "Down", |world| { /* move down */ });
/// kb.bind_many(WidgetId("list"), keys![KeyCode::Up, 'k'], "Up", |world| { /* move up */ });
/// ```
pub struct Keybindings {
    bindings: Vec<Binding>,
    any_key_handlers: Vec<AnyKeyHandler>,
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
            any_key_handlers: Vec::new(),
        }
    }

    /// Registers a keybinding for a specific widget.
    ///
    /// # Example
    ///
    /// ```ignore
    /// world.get_mut::<Keybindings>().bind(
    ///     WidgetId("list"),
    ///     KeyCode::Enter,
    ///     "Select item",
    ///     |world| {
    ///         world.get_mut::<MyState>().select();
    ///     },
    /// );
    /// ```
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

    /// Binds multiple keys to the same action.
    ///
    /// # Example
    ///
    /// ```ignore
    /// world.get_mut::<Keybindings>().bind_many(
    ///     WidgetId("list"),
    ///     [KeyCode::Char('j'), KeyCode::Down],
    ///     "Move down",
    ///     |world| {
    ///         world.get_mut::<MyState>().move_down();
    ///     },
    /// );
    /// ```
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

    /// Binds a handler that fires for any key press on the given widget
    /// when no specific binding matches.
    ///
    /// # Example
    ///
    /// ```ignore
    /// world.get_mut::<Keybindings>().bind_any(
    ///     WidgetId("input"),
    ///     |world, key| {
    ///         if let KeyBinding { code: KeyCode::Char(c), .. } = key {
    ///             world.get_mut::<InputState>().push(*c);
    ///         }
    ///     },
    /// );
    /// ```
    pub fn bind_any(
        &mut self,
        id: WidgetId,
        handler: impl Fn(&mut World, &KeyBinding) + Send + Sync + 'static,
    ) {
        self.any_key_handlers.push(AnyKeyHandler {
            id,
            handler: Box::new(handler),
        });
    }

    /// Dispatches a key event to matching keybindings.
    ///
    /// This method is called internally and should typically not be invoked directly.
    /// Use [`InputEvent::Key`](crate::InputEvent::Key) instead, which handles
    /// dispatching to both keybindings and other input handlers.
    ///
    /// Returns `true` if a matching binding was found and executed.
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

        for handler in &self.any_key_handlers {
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

    /// Returns keybindings grouped by name for the given widget IDs.
    /// Keys with the same name and widget ID are grouped together.
    #[must_use]
    pub fn display_for(&self, ids: &[WidgetId]) -> Vec<DisplayInfo> {
        use std::collections::HashMap;

        let mut groups: HashMap<(WidgetId, &'static str), Vec<KeyBinding>> = HashMap::new();
        let mut order: Vec<(WidgetId, &'static str)> = Vec::new();

        for binding in &self.bindings {
            if ids.contains(&binding.id) {
                let key = (binding.id, binding.name);
                if !groups.contains_key(&key) {
                    order.push(key);
                }
                groups.entry(key).or_default().push(binding.key);
            }
        }

        order
            .into_iter()
            .map(|(id, name)| DisplayInfo {
                keys: groups.remove(&(id, name)).unwrap_or_default(),
                id,
                name,
            })
            .collect()
    }

    /// Returns all keybindings grouped by name.
    /// Keys with the same name and widget ID are grouped together.
    #[must_use]
    pub fn display_all(&self) -> Vec<DisplayInfo> {
        use std::collections::HashMap;

        let mut groups: HashMap<(WidgetId, &'static str), Vec<KeyBinding>> = HashMap::new();
        let mut order: Vec<(WidgetId, &'static str)> = Vec::new();

        for binding in &self.bindings {
            let key = (binding.id, binding.name);
            if !groups.contains_key(&key) {
                order.push(key);
            }
            groups.entry(key).or_default().push(binding.key);
        }

        order
            .into_iter()
            .map(|(id, name)| DisplayInfo {
                keys: groups.remove(&(id, name)).unwrap_or_default(),
                id,
                name,
            })
            .collect()
    }
}

/// Represents a keyboard shortcut consisting of a key code and optional modifiers.
///
/// `KeyBinding` can be created from:
/// - A [`KeyCode`] directly (no modifiers)
/// - A `char` (uppercase letters automatically include Shift modifier)
/// - A [`KeyEvent`] reference
///
/// # Example
/// ```ignore
/// // Simple character key
/// let key: KeyBinding = 'j'.into();
///
/// // With modifiers
/// let ctrl_c = KeyBinding::ctrl('c');
/// let shift_tab = KeyBinding::new(KeyCode::Tab, KeyModifiers::SHIFT);
///
/// // Uppercase automatically includes Shift
/// let shift_h: KeyBinding = 'H'.into();
/// ```
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

    /// Returns a compact display string using symbols for modifiers.
    /// - Shift → `⇧`
    /// - Ctrl → `⌃`
    /// - Alt → `⌥`
    ///
    /// Examples: `⇧Tab`, `⌃c`, `⌥x`, `⇧H`
    #[must_use]
    pub fn display_compact(&self) -> String {
        let mut parts = String::new();

        if self.modifiers.contains(KeyModifiers::CONTROL) {
            parts.push('⌃');
        }
        if self.modifiers.contains(KeyModifiers::ALT) {
            parts.push('⌥');
        }
        if self.modifiers.contains(KeyModifiers::SHIFT) {
            parts.push('⇧');
        }

        let key = match self.code {
            KeyCode::Char(c) => {
                if c == ' ' {
                    "Space".to_string()
                } else {
                    c.to_ascii_lowercase().to_string()
                }
            }
            KeyCode::Up => "↑".to_string(),
            KeyCode::Down => "↓".to_string(),
            KeyCode::Left => "←".to_string(),
            KeyCode::Right => "→".to_string(),
            KeyCode::Enter => "⏎".to_string(),
            KeyCode::Esc => "Esc".to_string(),
            KeyCode::BackTab | KeyCode::Tab => "Tab".to_string(),
            KeyCode::Backspace => "⌫".to_string(),
            KeyCode::Delete => "Del".to_string(),
            KeyCode::Home => "Home".to_string(),
            KeyCode::End => "End".to_string(),
            KeyCode::PageUp => "PgUp".to_string(),
            KeyCode::PageDown => "PgDn".to_string(),
            KeyCode::F(n) => format!("F{n}"),
            _ => format!("{:?}", self.code),
        };

        parts.push_str(&key);
        parts
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
                c.to_ascii_lowercase().to_string()
            }
            KeyCode::Up => "↑".to_string(),
            KeyCode::Down => "↓".to_string(),
            KeyCode::Left => "←".to_string(),
            KeyCode::Right => "→".to_string(),
            KeyCode::Enter => "Enter".to_string(),
            KeyCode::Esc => "Esc".to_string(),
            KeyCode::Tab => "Tab".to_string(),
            KeyCode::BackTab => "Shift+Tab".to_string(),
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
        if c.is_ascii_uppercase() {
            Self::new(KeyCode::Char(c), KeyModifiers::SHIFT)
        } else {
            Self::key(KeyCode::Char(c))
        }
    }
}

/// A collection of keybindings for use with [`Keybindings::bind_many`].
///
/// Can be created using the [`keys!`] macro or converted from a `Vec<KeyBinding>`.
///
/// # Example
/// ```ignore
/// // Using the keys! macro
/// kb.bind_many(id, keys![KeyCode::Up, 'k'], "Up", |world| { });
///
/// // From a vector
/// let keys: Keys = vec![KeyBinding::key(KeyCode::Up), 'k'.into()].into();
/// ```
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

/// Information about keybindings for display purposes.
///
/// Groups multiple keys that perform the same action together,
/// along with the widget ID and action name for UI presentation.
#[derive(Debug)]
pub struct DisplayInfo {
    pub keys: Vec<KeyBinding>,
    pub id: WidgetId,
    pub name: &'static str,
}

impl DisplayInfo {
    /// Returns a formatted string of all keys, e.g. "j, k, ↓"
    ///
    /// # Example
    /// ```ignore
    /// // Returns "Shift+h, j, k"
    /// info.keys_display()
    /// ```
    #[must_use]
    pub fn keys_display(&self) -> String {
        self.keys_display_with(KeyBinding::display)
    }

    /// Returns a compact formatted string of all keys, e.g. "⇧h, ⌃c, ↓"
    ///
    /// # Example
    /// ```ignore
    /// // Returns "⇧h, j, k"
    /// info.keys_display_compact()
    /// ```
    #[must_use]
    pub fn keys_display_compact(&self) -> String {
        self.keys_display_with(KeyBinding::display_compact)
    }

    /// Returns a formatted string of all keys using a custom formatter.
    ///
    /// # Example
    /// ```ignore
    /// // Compact: "H, j, k" (Shift+letter shown as uppercase)
    /// info.keys_display_with(KeyBinding::display_compact)
    ///
    /// // Custom formatter: "[↑], [↓]"
    /// info.keys_display_with(|k| format!("[{}]", k.display()))
    /// ```
    #[must_use]
    pub fn keys_display_with(&self, formatter: impl Fn(&KeyBinding) -> String) -> String {
        self.keys
            .iter()
            .map(formatter)
            .collect::<Vec<_>>()
            .join(", ")
    }
}
