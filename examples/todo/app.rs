use crate::dialog::{self, DialogState};
use crate::help;
use crate::theme::Theme;
use crate::todo::{self, TodoState};
use crossterm::event::KeyCode;
use ratatui::Frame;
use tui_world::prelude::*;

pub const GLOBAL_ID: WidgetId = WidgetId("Global");

#[derive(Default)]
pub struct AppState {
    pub should_quit: bool,
    pub help_open: bool,
}

pub fn setup(world: &mut World) {
    world.insert(Theme::default());
    world.insert(AppState::default());
    world.insert(TodoState::default());
    world.insert(DialogState::default());
    world.insert(Focus::new(todo::TODO_LIST_ID));

    setup_keybindings(world);
}

fn setup_keybindings(world: &mut World) {
    let kb = world.get_mut::<Keybindings>();

    kb.bind(GLOBAL_ID, KeyBinding::ctrl('c'), "Quit", |world| {
        world.get_mut::<AppState>().should_quit = true;
    });

    kb.bind(
        GLOBAL_ID,
        KeyBinding::key(KeyCode::Char('?')),
        "Help",
        |world| {
            help::toggle(world);
        },
    );

    todo::setup_keybindings(kb);
    dialog::setup_keybindings(kb);
}

pub fn render(frame: &mut Frame, world: &mut World) {
    let area = frame.area();

    todo::render(frame, world, area);

    if world.get::<DialogState>().open {
        dialog::render(frame, world, area);
    }

    if world.get::<AppState>().help_open {
        help::render(frame, world, area);
    }
}

pub fn get_active_ids(world: &World) -> Vec<WidgetId> {
    let mut active = vec![GLOBAL_ID];

    if let Some(id) = world.get::<Focus>().id {
        active.push(id);
    }

    active
}
