use std::collections::HashMap;

use crossterm::event::{self, Event as CEvent, KeyCode, KeyModifiers};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders},
};
use tui_world::prelude::*;

const TEXT_ID: WidgetId = WidgetId("text");

fn main() -> anyhow::Result<()> {
    let mut terminal = ratatui::init();
    crossterm::execute!(std::io::stdout(), crossterm::event::EnableMouseCapture)?;

    let mut world = World::default();
    world.insert(Selections::default());

    world.get_mut::<Keybindings>().bind(
        TEXT_ID,
        KeyBinding::new(KeyCode::Char('c'), KeyModifiers::CONTROL),
        "Quit",
        |w| w.insert(Quit),
    );

    loop {
        terminal.draw(|frame| {
            let block = Block::default()
                .borders(Borders::ALL)
                .title(" Select text (Ctrl+C to quit) ");
            let inner = block.inner(frame.area());
            frame.render_widget(block, frame.area());

            SelectableText::new(TEXT_ID, "Click and drag to select this text.")
                .style(Style::default().fg(Color::White))
                .selection_style(Style::default().fg(Color::White).bg(Color::Blue))
                .render(inner, frame.buffer_mut(), &mut world);
        })?;

        if world.exists::<Quit>() {
            break;
        }

        if event::poll(std::time::Duration::from_millis(16))? {
            match event::read()? {
                CEvent::Key(key) => InputEvent::Key(key).handle(&mut world, &[TEXT_ID]),
                CEvent::Mouse(mouse) => InputEvent::Mouse(mouse).handle(&mut world, &[TEXT_ID]),
                _ => {}
            }
        }
    }

    crossterm::execute!(std::io::stdout(), crossterm::event::DisableMouseCapture)?;
    ratatui::restore();
    Ok(())
}

struct Quit;

/// Stores selection state for selectable text widgets
#[derive(Default)]
pub struct Selections {
    selections: HashMap<WidgetId, Option<(usize, usize)>>,
    anchors: HashMap<WidgetId, usize>,
    text_areas: HashMap<WidgetId, (u16, u16, String)>,
}

impl Selections {
    pub fn get(&self, id: WidgetId) -> Option<(usize, usize)> {
        self.selections.get(&id).copied().flatten()
    }

    pub fn set(&mut self, id: WidgetId, selection: Option<(usize, usize)>) {
        self.selections.insert(id, selection);
    }

    pub fn set_anchor(&mut self, id: WidgetId, anchor: usize) {
        self.anchors.insert(id, anchor);
    }

    pub fn get_anchor(&self, id: WidgetId) -> Option<usize> {
        self.anchors.get(&id).copied()
    }

    pub fn set_text_area(&mut self, id: WidgetId, x: u16, y: u16, text: String) {
        self.text_areas.insert(id, (x, y, text));
    }

    pub fn get_text_area(&self, id: WidgetId) -> Option<&(u16, u16, String)> {
        self.text_areas.get(&id)
    }

    pub fn coords_to_index(&self, id: WidgetId, x: u16, y: u16) -> Option<usize> {
        let (area_x, area_y, text) = self.get_text_area(id)?;
        if y != *area_y {
            return None;
        }
        let offset = x.saturating_sub(*area_x) as usize;
        Some(offset.min(text.len()))
    }
}

pub struct SelectableText {
    id: WidgetId,
    text: String,
    style: Style,
    selection_style: Style,
}

impl SelectableText {
    pub fn new(id: WidgetId, text: impl Into<String>) -> Self {
        Self {
            id,
            text: text.into(),
            style: Style::default(),
            selection_style: Style::default().bg(Color::Blue),
        }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn selection_style(mut self, style: Style) -> Self {
        self.selection_style = style;
        self
    }

    pub fn render(self, area: Rect, buf: &mut Buffer, world: &mut World) {
        {
            let selections = world.get_mut::<Selections>();
            selections.set_text_area(self.id, area.x, area.y, self.text.clone());
        }

        let selection = world.get::<Selections>().get(self.id);

        let chars: Vec<char> = self.text.chars().collect();
        for (i, ch) in chars.iter().enumerate() {
            if i >= area.width as usize {
                break;
            }

            let style = if let Some((start, end)) = selection {
                let (start, end) = if start <= end {
                    (start, end)
                } else {
                    (end, start)
                };
                if i >= start && i < end {
                    self.selection_style
                } else {
                    self.style
                }
            } else {
                self.style
            };

            buf[(area.x + i as u16, area.y)]
                .set_char(*ch)
                .set_style(style);
        }

        let id = self.id;

        world.get_mut::<Pointer>().set(id, area);

        world.get_mut::<Pointer>().on_down(id, move |w, _, x, y| {
            let index = w.get::<Selections>().coords_to_index(id, x, y);
            if let Some(idx) = index {
                let selections = w.get_mut::<Selections>();
                selections.set_anchor(id, idx);
                selections.set(id, Some((idx, idx)));
            }
        });

        world.get_mut::<Pointer>().on_drag(id, move |w, _, x, y| {
            let index = w.get::<Selections>().coords_to_index(id, x, y);
            let anchor = w.get::<Selections>().get_anchor(id);
            if let (Some(idx), Some(anchor)) = (index, anchor) {
                w.get_mut::<Selections>().set(id, Some((anchor, idx)));
            }
        });

        world.get_mut::<Pointer>().on_up(id, move |w, _, x, y| {
            let index = w.get::<Selections>().coords_to_index(id, x, y);
            let anchor = w.get::<Selections>().get_anchor(id);
            if let (Some(idx), Some(anchor)) = (index, anchor) {
                w.get_mut::<Selections>().set(id, Some((anchor, idx)));
            }
        });
    }
}
