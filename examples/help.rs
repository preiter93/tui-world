use crossterm::event::{self, Event as CEvent, KeyCode};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};
use std::collections::BTreeMap;
use tui_world::prelude::*;

const GLOBAL_ID: WidgetId = WidgetId("Global");
const CONTENT_ID: WidgetId = WidgetId("Content");

#[derive(Default)]
struct AppState {
    should_quit: bool,
    help_open: bool,
    counter: i32,
}

fn setup(world: &mut World) {
    world.insert(AppState::default());
    world.insert(Focus::new(CONTENT_ID));

    let kb = world.get_mut::<Keybindings>();

    kb.bind(GLOBAL_ID, KeyBinding::ctrl('c'), "Quit", |world| {
        world.get_mut::<AppState>().should_quit = true;
    });

    kb.bind(
        GLOBAL_ID,
        KeyBinding::key(KeyCode::Char('?')),
        "Toggle help",
        |world| {
            world.get_mut::<AppState>().help_open ^= true;
        },
    );

    kb.bind(
        CONTENT_ID,
        KeyBinding::key(KeyCode::Up),
        "Increment",
        |world| {
            world.get_mut::<AppState>().counter += 1;
        },
    );

    kb.bind(
        CONTENT_ID,
        KeyBinding::key(KeyCode::Down),
        "Decrement",
        |world| {
            world.get_mut::<AppState>().counter -= 1;
        },
    );

    kb.bind(
        CONTENT_ID,
        KeyBinding::key(KeyCode::Char('r')),
        "Reset",
        |world| {
            world.get_mut::<AppState>().counter = 0;
        },
    );
}

fn get_active_ids(world: &World) -> Vec<WidgetId> {
    let mut active = vec![GLOBAL_ID];
    if let Some(id) = world.get::<Focus>().id {
        active.push(id);
    }
    active
}

fn render(frame: &mut Frame, world: &mut World) {
    let area = frame.area();
    let state = world.get::<AppState>();

    let accent = Color::Rgb(122, 162, 247);
    let fg = Color::Rgb(192, 202, 245);
    let muted = Color::Rgb(86, 95, 137);

    let block = Block::default()
        .title(" Help Example ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(accent));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let content = vec![
        Line::from(Span::styled(
            format!("Counter: {}", state.counter),
            Style::default().fg(fg),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Press ? to toggle help",
            Style::default().fg(muted),
        )),
    ];

    frame.render_widget(Paragraph::new(content), inner);

    if state.help_open {
        render_help(frame, world, area);
    }
}

fn render_help(frame: &mut Frame, world: &mut World, area: Rect) {
    let accent = Color::Rgb(122, 162, 247);
    let fg = Color::Rgb(192, 202, 245);
    let success = Color::Rgb(158, 206, 106);

    let width = 30.min(area.width.saturating_sub(4));
    let height = 12.min(area.height.saturating_sub(4));

    let [_, h_center, _] = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(width),
        Constraint::Fill(1),
    ])
    .areas(area);

    let [_, dialog_area, _] = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(height),
        Constraint::Fill(1),
    ])
    .areas(h_center);

    frame.render_widget(ratatui::widgets::Clear, dialog_area);

    let block = Block::default()
        .title(" Keybindings ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(accent));

    let inner = block.inner(dialog_area);
    frame.render_widget(block, dialog_area);

    let active = get_active_ids(world);
    let keybindings = world.get::<Keybindings>();
    let display = keybindings.display_for(&active);

    let mut lines: Vec<Line> = Vec::new();
    let mut groups: BTreeMap<WidgetId, BTreeMap<&'static str, Vec<&DisplayInfo>>> = BTreeMap::new();

    for info in &display {
        groups
            .entry(info.id)
            .or_default()
            .entry(info.name)
            .or_default()
            .push(info);
    }

    for (id, commands) in groups {
        if !lines.is_empty() {
            lines.push(Line::from(""));
        }
        lines.push(Line::from(Span::styled(
            format!("[{}]", id.0),
            Style::default().fg(accent),
        )));

        for (name, infos) in commands {
            let keys = infos
                .iter()
                .map(|i| i.key.to_string())
                .collect::<Vec<_>>()
                .join("/");

            lines.push(Line::from(vec![
                Span::styled(format!("{:>10}", keys), Style::default().fg(success)),
                Span::raw("  "),
                Span::styled(name, Style::default().fg(fg)),
            ]));
        }
    }

    frame.render_widget(Paragraph::new(lines), inner);
}

fn main() -> anyhow::Result<()> {
    let mut terminal = ratatui::init();

    let mut world = World::default();
    setup(&mut world);

    loop {
        terminal.draw(|frame| render(frame, &mut world))?;

        if event::poll(std::time::Duration::from_millis(16))? {
            let active = get_active_ids(&world);

            if let CEvent::Key(key) = event::read()? {
                Event::Key(key).handle(&mut world, &active);
            }
        }

        if world.get::<AppState>().should_quit {
            break;
        }
    }

    ratatui::restore();
    Ok(())
}
