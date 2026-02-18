use crossterm::event::{self, Event as CEvent, KeyCode};
use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
};
use tui_world::keys;
use tui_world::prelude::*;

const GLOBAL_ID: WidgetId = WidgetId("Global");
const TODO_LIST_ID: WidgetId = WidgetId("Todo");
const DIALOG_ID: WidgetId = WidgetId("Dialog");

#[derive(Default)]
struct AppState {
    should_quit: bool,
}

struct Todo {
    text: String,
    done: bool,
}

struct TodoState {
    todos: Vec<Todo>,
    selected: usize,
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
    fn move_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    fn move_down(&mut self) {
        if self.selected + 1 < self.todos.len() {
            self.selected += 1;
        }
    }

    fn toggle_selected(&mut self) {
        if let Some(todo) = self.todos.get_mut(self.selected) {
            todo.done = !todo.done;
        }
    }

    fn delete_selected(&mut self) {
        if !self.todos.is_empty() {
            self.todos.remove(self.selected);
            if self.selected >= self.todos.len() && self.selected > 0 {
                self.selected -= 1;
            }
        }
    }
}

struct DialogState {
    open: bool,
    input: String,
    title: &'static str,
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
    fn open(&mut self, title: &'static str) {
        self.open = true;
        self.input.clear();
        self.title = title;
    }

    fn close(&mut self) {
        self.open = false;
        self.input.clear();
    }

    fn push(&mut self, c: char) {
        self.input.push(c);
    }

    fn pop(&mut self) {
        self.input.pop();
    }

    fn take_input(&mut self) -> String {
        std::mem::take(&mut self.input)
    }
}

fn setup(world: &mut World) {
    world.insert(AppState::default());
    world.insert(TodoState::default());
    world.insert(DialogState::default());
    world.insert(Focus::new(TODO_LIST_ID));

    setup_keybindings(world);
    setup_pointer(world);
}

fn setup_keybindings(world: &mut World) {
    let kb = world.get_mut::<Keybindings>();

    kb.bind(GLOBAL_ID, KeyBinding::ctrl('c'), "Quit", |world| {
        world.get_mut::<AppState>().should_quit = true;
    });

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

    kb.bind_any(DIALOG_ID, |world, key| {
        if let KeyCode::Char(c) = key.code {
            world.get_mut::<DialogState>().push(c);
        }
    });
}

fn setup_pointer(world: &mut World) {
    world
        .get_mut::<Pointer>()
        .on_click(TODO_LIST_ID, |world, area, x, y| {
            let row = (y - area.y) as usize;
            let col = (x - area.x) as usize;
            let state = world.get_mut::<TodoState>();

            if row >= state.todos.len() {
                return;
            }

            if (1..=3).contains(&col) {
                state.todos[row].done ^= true;
            } else {
                state.selected = row;
            }
        });
}

fn render(frame: &mut Frame, world: &mut World) {
    let area = frame.area();

    render_todo(frame, world, area);

    if world.get::<DialogState>().open {
        render_dialog(frame, world, area);
    }
}

fn render_todo(frame: &mut Frame, world: &mut World, area: Rect) {
    let state = world.get::<TodoState>();

    let fg = Color::Rgb(192, 202, 245);
    let muted = Color::Rgb(86, 95, 137);
    let accent = Color::Rgb(122, 162, 247);
    let highlight = Color::Rgb(51, 70, 124);

    let text_style = Style::default().fg(fg);
    let text_muted_style = Style::default().fg(muted);
    let border_style = Style::default().fg(accent);
    let selected_style = Style::default().fg(fg).bg(highlight);

    let block = Block::default()
        .title(" Todos ")
        .borders(Borders::ALL)
        .border_style(border_style)
        .border_type(BorderType::Rounded);

    let inner = block.inner(area);

    let mut lines: Vec<Line> = Vec::new();

    for (i, todo) in state.todos.iter().enumerate() {
        let checkbox = if todo.done { "[x]" } else { "[ ]" };
        let style = if i == state.selected {
            selected_style
        } else if todo.done {
            text_muted_style
        } else {
            text_style
        };

        lines.push(Line::from(Span::styled(
            format!(" {} {}", checkbox, todo.text),
            style,
        )));
    }

    if lines.is_empty() {
        lines.push(Line::from(Span::styled("  No todos", text_muted_style)));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        " Press 'a' to add a todo",
        text_muted_style,
    )));
    lines.push(Line::from(Span::styled(
        " Press 'Enter' to toggle",
        text_muted_style,
    )));
    lines.push(Line::from(Span::styled(
        " And try clicking around",
        text_muted_style,
    )));

    frame.render_widget(block, area);
    frame.render_widget(Paragraph::new(lines), inner);

    world.get_mut::<Pointer>().set(TODO_LIST_ID, inner);
}

fn render_dialog(frame: &mut Frame, world: &mut World, area: Rect) {
    let state = world.get::<DialogState>();

    let width = 40.min(area.width.saturating_sub(4));
    let height = 3;

    let [dialog_area] = Layout::horizontal([Constraint::Length(width)])
        .flex(Flex::Center)
        .areas(area);
    let [dialog_area] = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .areas(dialog_area);

    let accent = Color::Rgb(122, 162, 247);
    let fg = Color::Rgb(192, 202, 245);

    let block = Block::default()
        .title(format!(" {} ", state.title))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(accent))
        .border_type(BorderType::Rounded);

    let inner = block.inner(dialog_area);

    let input_text = format!("{}_", state.input);
    let input = Paragraph::new(input_text).style(Style::default().fg(fg));

    frame.render_widget(Clear, dialog_area);
    frame.render_widget(block, dialog_area);
    frame.render_widget(input, inner);

    world.get_mut::<Pointer>().set(DIALOG_ID, inner);
}

fn main() -> anyhow::Result<()> {
    let mut terminal = ratatui::init();
    crossterm::execute!(std::io::stdout(), crossterm::event::EnableMouseCapture)?;

    let mut world = World::default();
    setup(&mut world);

    loop {
        terminal.draw(|frame| render(frame, &mut world))?;

        if event::poll(std::time::Duration::from_millis(16))? {
            let mut active = vec![GLOBAL_ID];
            if let Some(id) = world.get::<Focus>().id {
                active.push(id);
            }

            match event::read()? {
                CEvent::Key(key) => InputEvent::Key(key).handle(&mut world, &active),
                CEvent::Mouse(mouse) => InputEvent::Mouse(mouse).handle(&mut world, &active),
                _ => {}
            }
        }

        if world.get::<AppState>().should_quit {
            break;
        }
    }

    crossterm::execute!(std::io::stdout(), crossterm::event::DisableMouseCapture)?;
    ratatui::restore();

    Ok(())
}
