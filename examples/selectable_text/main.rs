use crossterm::event::{self, Event as CEvent, KeyCode, KeyModifiers};
use ratatui::{
    Frame,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};
use tui_world::prelude::*;

const TEXT_ID: WidgetId = WidgetId("text");

const TEXT: &str = "Click and drag to select this text.";

#[derive(Default)]
struct Selection {
    start: Option<usize>,
    end: Option<usize>,
    area: Option<Area>,
}

impl Selection {
    fn range(&self) -> Option<(usize, usize)> {
        let (s, e) = (self.start?, self.end?);
        Some((s.min(e), s.max(e)))
    }

    fn to_index(&self, x: u16, y: u16) -> usize {
        let Some(area) = self.area else { return 0 };
        let rx = x.saturating_sub(area.x) as usize;
        let ry = y.saturating_sub(area.y) as usize;
        (ry * area.width as usize + rx).min(TEXT.len())
    }
}

fn main() -> anyhow::Result<()> {
    let mut terminal = ratatui::init();
    crossterm::execute!(std::io::stdout(), crossterm::event::EnableMouseCapture)?;

    let mut world = World::default();
    world.insert(Selection::default());

    world.get_mut::<Pointer>().on_down(TEXT_ID, |w, x, y| {
        let idx = w.get::<Selection>().to_index(x, y);
        let sel = w.get_mut::<Selection>();
        sel.start = Some(idx);
        sel.end = Some(idx);
    });

    world.get_mut::<Pointer>().on_drag(TEXT_ID, |w, x, y| {
        let idx = w.get::<Selection>().to_index(x, y);
        w.get_mut::<Selection>().end = Some(idx);
    });

    world.get_mut::<Keybindings>().bind(
        TEXT_ID,
        KeyBinding::new(KeyCode::Char('c'), KeyModifiers::CONTROL),
        "Quit",
        |w| w.insert(Quit),
    );

    world.get_mut::<Focus>().set(TEXT_ID);

    loop {
        terminal.draw(|f| render(f, &mut world))?;

        if world.exists::<Quit>() {
            break;
        }

        if event::poll(std::time::Duration::from_millis(16))? {
            match event::read()? {
                CEvent::Key(key) => Event::Key(key).handle(&mut world, &[TEXT_ID]),
                CEvent::Mouse(mouse) => Event::Mouse(mouse).handle(&mut world, &[TEXT_ID]),
                _ => {}
            }
        }
    }

    crossterm::execute!(std::io::stdout(), crossterm::event::DisableMouseCapture)?;
    ratatui::restore();
    Ok(())
}

struct Quit;

fn render(frame: &mut Frame, world: &mut World) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Select text ");
    let inner = block.inner(frame.area());
    frame.render_widget(block, frame.area());

    let selection = world.get::<Selection>().range();
    let normal = Style::default().fg(Color::White);
    let selected = Style::default().fg(Color::White).bg(Color::Blue);

    let spans: Vec<Span> = TEXT
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let is_sel = selection.map(|(s, e)| i >= s && i < e).unwrap_or(false);
            Span::styled(c.to_string(), if is_sel { selected } else { normal })
        })
        .collect();

    frame.render_widget(Paragraph::new(Line::from(spans)), inner);

    world.get_mut::<Selection>().area = Some(inner.into());
    world.get_mut::<Pointer>().set(TEXT_ID, inner);
}
