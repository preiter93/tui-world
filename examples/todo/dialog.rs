use crate::theme::Theme;
use crate::todo::{TODO_LIST_ID, Todo, TodoState};
use crossterm::event::KeyCode;
use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout, Rect},
    widgets::{Block, Borders, Clear, Paragraph},
};
use tui_world::prelude::*;

pub const DIALOG_ID: WidgetId = WidgetId("Dialog");

pub struct DialogState {
    pub open: bool,
    pub input: String,
    pub title: &'static str,
}

impl Default for DialogState {
    fn default() -> Self {
        Self {
            open: false,
            input: String::new(),
            title: "Input",
        }
    }
}

impl DialogState {
    pub fn open(&mut self, title: &'static str) {
        self.open = true;
        self.input.clear();
        self.title = title;
    }

    pub fn close(&mut self) {
        self.open = false;
        self.input.clear();
    }

    pub fn push(&mut self, c: char) {
        self.input.push(c);
    }

    pub fn pop(&mut self) {
        self.input.pop();
    }

    pub fn take_input(&mut self) -> String {
        std::mem::take(&mut self.input)
    }
}

pub fn setup_keybindings(kb: &mut Keybindings) {
    kb.bind(
        DIALOG_ID,
        KeyBinding::key(KeyCode::Esc),
        "Cancel",
        |world| {
            world.get_mut::<DialogState>().close();
            world.get_mut::<Focus>().set(TODO_LIST_ID);
        },
    );

    kb.bind(
        DIALOG_ID,
        KeyBinding::key(KeyCode::Enter),
        "Confirm",
        |world| {
            let input = world.get_mut::<DialogState>().take_input();
            if !input.is_empty() {
                world.get_mut::<TodoState>().todos.push(Todo {
                    text: input,
                    done: false,
                });
                let len = world.get::<TodoState>().todos.len();
                world.get_mut::<TodoState>().selected = len - 1;
            }
            world.get_mut::<DialogState>().close();
            world.get_mut::<Focus>().set(TODO_LIST_ID);
        },
    );

    kb.bind(
        DIALOG_ID,
        KeyBinding::key(KeyCode::Backspace),
        "Delete char",
        |world| {
            world.get_mut::<DialogState>().pop();
        },
    );

    kb.catch_all(Context::Widget(DIALOG_ID), |world, key| {
        if let KeyCode::Char(c) = key.code {
            world.get_mut::<DialogState>().push(c);
        }
    });
}

pub fn render(frame: &mut Frame, world: &mut World, area: Rect) {
    let theme = world.get::<Theme>();
    let state = world.get::<DialogState>();

    let width = 40.min(area.width.saturating_sub(4));
    let height = 3;

    let [dialog_area] = Layout::horizontal([Constraint::Length(width)])
        .flex(Flex::Center)
        .areas(area);
    let [dialog_area] = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .areas(dialog_area);

    let block = Block::default()
        .title(format!(" {} ", state.title))
        .borders(Borders::ALL)
        .border_style(theme.border_focused)
        .border_type(theme.border_type);

    let inner = block.inner(dialog_area);

    let input_text = format!("{}_", state.input);
    let input = Paragraph::new(input_text).style(theme.text);

    frame.render_widget(Clear, dialog_area);
    frame.render_widget(block, dialog_area);
    frame.render_widget(input, inner);

    world.get_mut::<Pointer>().set(DIALOG_ID, inner);
}
