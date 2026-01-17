use crate::app::AppState;
use crate::theme::Theme;
use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};
use std::collections::BTreeMap;
use tui_world::prelude::*;

const HELP_BACKDROP_ID: WidgetId = WidgetId("help_backdrop");

pub fn toggle(world: &mut World) {
    let is_open = world.get::<AppState>().help_open;
    if is_open {
        close(world);
    } else {
        open(world);
    }
}

pub fn open(world: &mut World) {
    world.get_mut::<AppState>().help_open = true;

    let area = world.get::<AppState>().area;
    let dialog_area = center_rect(area, 40, 15);

    world.get_mut::<Layout>().set(HELP_BACKDROP_ID, area);
    world
        .get_mut::<Mouse>()
        .on_click(HELP_BACKDROP_ID, move |world, x, y| {
            if !dialog_area.contains((x, y).into()) {
                close(world);
            }
        });
}

pub fn close(world: &mut World) {
    world.get_mut::<AppState>().help_open = false;
    world.get_mut::<Layout>().remove(HELP_BACKDROP_ID);
    world.get_mut::<Mouse>().remove(HELP_BACKDROP_ID);
}

pub fn render(frame: &mut Frame, world: &World, area: Rect) {
    let theme = world.get::<Theme>();
    let dialog_area = center_rect(area, 40, 15);

    frame.render_widget(Clear, dialog_area);

    let block = Block::default()
        .title(" Keybindings ")
        .borders(Borders::ALL)
        .border_style(theme.border);

    let inner = block.inner(dialog_area);
    frame.render_widget(block, dialog_area);

    let focus = world.get::<Focus>().get();
    let keybindings = world.get::<Keybindings>();
    let display = keybindings.display_for(focus);

    let mut lines: Vec<Line> = Vec::new();

    // Group bindings by (context, name)
    let mut widget_groups: BTreeMap<&'static str, Vec<&DisplayInfo>> = BTreeMap::new();
    let mut global_groups: BTreeMap<&'static str, Vec<&DisplayInfo>> = BTreeMap::new();
    let mut widget_label = "";

    for info in &display {
        match info.context {
            Context::Widget(id) => {
                widget_label = id.0;
                widget_groups.entry(info.name).or_default().push(info);
            }
            Context::Global => {
                global_groups.entry(info.name).or_default().push(info);
            }
        }
    }

    if !widget_groups.is_empty() {
        lines.push(Line::from(Span::styled(
            format!("[{}]", widget_label),
            theme.title,
        )));
        for (name, infos) in &widget_groups {
            let keys = infos
                .iter()
                .map(|i| i.key.to_string())
                .collect::<Vec<_>>()
                .join("/");
            lines.push(Line::from(vec![
                Span::styled(format!("{:>10}", keys), theme.keybinding_key),
                Span::raw("  "),
                Span::styled(*name, theme.text),
            ]));
        }
        lines.push(Line::from(""));
    }

    if !global_groups.is_empty() {
        lines.push(Line::from(Span::styled("[Global]", theme.title)));
        for (name, infos) in &global_groups {
            let keys = infos
                .iter()
                .map(|i| i.key.to_string())
                .collect::<Vec<_>>()
                .join("/");
            lines.push(Line::from(vec![
                Span::styled(format!("{:>10}", keys), theme.keybinding_key),
                Span::raw("  "),
                Span::styled(*name, theme.text),
            ]));
        }
    }

    frame.render_widget(Paragraph::new(lines), inner);
}

fn center_rect(area: Rect, width: u16, height: u16) -> Rect {
    let width = width.min(area.width.saturating_sub(4));
    let height = height.min(area.height.saturating_sub(4));

    let [_, h_center, _] = ratatui::layout::Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(width),
        Constraint::Fill(1),
    ])
    .areas(area);

    let [_, dialog, _] = ratatui::layout::Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(height),
        Constraint::Fill(1),
    ])
    .areas(h_center);

    dialog
}
