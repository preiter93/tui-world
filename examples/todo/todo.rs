use crate::app::AppState;
use crate::dialog::{DIALOG_ID, DialogState};
use crate::theme::Theme;
use crossterm::event::KeyCode;
use ratatui::{
    Frame,
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};
use tui_world::keys;
use tui_world::prelude::*;

pub const TODO_LIST_ID: WidgetId = WidgetId("Todo");

pub struct Todo {
    pub text: String,
    pub done: bool,
}

pub struct TodoState {
    pub todos: Vec<Todo>,
    pub selected: usize,
}

impl Default for TodoState {
    fn default() -> Self {
        Self {
            todos: vec![
                Todo {
                    text: "Buy groceries".into(),
                    done: false,
                },
                Todo {
                    text: "Get a coffee".into(),
                    done: false,
                },
                Todo {
                    text: "Write some code".into(),
                    done: false,
                },
                Todo {
                    text: "Go for a run".into(),
                    done: false,
                },
            ],
            selected: 0,
        }
    }
}

impl TodoState {
    pub fn move_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.selected + 1 < self.todos.len() {
            self.selected += 1;
        }
    }

    pub fn toggle_selected(&mut self) {
        if let Some(todo) = self.todos.get_mut(self.selected) {
            todo.done = !todo.done;
        }
    }

    pub fn delete_selected(&mut self) {
        if !self.todos.is_empty() {
            self.todos.remove(self.selected);
            if self.selected >= self.todos.len() && self.selected > 0 {
                self.selected -= 1;
            }
        }
    }
}

pub fn setup_keybindings(kb: &mut Keybindings) {
    kb.bind_many(TODO_LIST_ID, keys![KeyCode::Up, 'k'], "Up", |world| {
        world.get_mut::<TodoState>().move_up();
    });

    kb.bind_many(TODO_LIST_ID, keys![KeyCode::Down, 'j'], "Down", |world| {
        world.get_mut::<TodoState>().move_down();
    });

    kb.bind_many(
        TODO_LIST_ID,
        keys![KeyCode::Enter, ' '],
        "Toggle",
        |world| {
            world.get_mut::<TodoState>().toggle_selected();
        },
    );

    kb.bind(TODO_LIST_ID, 'd', "Delete", |world| {
        world.get_mut::<TodoState>().delete_selected();
    });

    kb.bind(TODO_LIST_ID, 'a', "Add", |world| {
        world.get_mut::<DialogState>().open("Add Todo");
        world.get_mut::<Focus>().set(DIALOG_ID);
    });
}

pub fn setup_click_handler(world: &mut World) {
    world
        .get_mut::<Mouse>()
        .on_click(TODO_LIST_ID, |world, _x, y| {
            let area = world.get::<AppState>().area;
            let inner_y = area.y + 1;

            let clicked_index = y.saturating_sub(inner_y) as usize;
            let todos_len = world.get::<TodoState>().todos.len();

            if clicked_index < todos_len
                && let Some(todo) = world.get_mut::<TodoState>().todos.get_mut(clicked_index)
            {
                todo.done = !todo.done;
            }
        });
}

pub fn render(frame: &mut Frame, world: &mut World, area: Rect) {
    let theme = world.get::<Theme>();
    let state = world.get::<TodoState>();

    let block = Block::default()
        .title(" Todos ")
        .borders(Borders::ALL)
        .border_style(theme.border)
        .border_type(theme.border_type);

    let inner = block.inner(area);

    let mut lines: Vec<Line> = Vec::new();

    for (i, todo) in state.todos.iter().enumerate() {
        let checkbox = if todo.done { "[x]" } else { "[ ]" };
        let style = if i == state.selected {
            theme.selected
        } else if todo.done {
            theme.text_muted
        } else {
            theme.text
        };

        lines.push(Line::from(Span::styled(
            format!(" {} {}", checkbox, todo.text),
            style,
        )));
    }

    if lines.is_empty() {
        lines.push(Line::from(Span::styled("  No todos", theme.text_muted)));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        " Press ? for help",
        theme.text_muted,
    )));

    frame.render_widget(block, area);
    frame.render_widget(Paragraph::new(lines), inner);

    world.get_mut::<HitMap>().set(TODO_LIST_ID, inner);
}
