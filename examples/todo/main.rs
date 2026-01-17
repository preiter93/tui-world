mod app;
mod dialog;
mod help;
mod theme;
mod todo;

use crossterm::event::{self, Event as CEvent};
use tui_world::prelude::*;

fn main() -> anyhow::Result<()> {
    let mut terminal = ratatui::init();
    crossterm::execute!(std::io::stdout(), crossterm::event::EnableMouseCapture)?;

    let mut world = World::default();
    app::setup(&mut world);

    loop {
        terminal.draw(|frame| app::render(frame, &mut world))?;

        if event::poll(std::time::Duration::from_millis(16))? {
            match event::read()? {
                CEvent::Key(key) => Event::Key(key).handle(&mut world),
                CEvent::Mouse(mouse) => Event::Mouse(mouse).handle(&mut world),
                _ => {}
            }
        }

        if world.get::<app::AppState>().should_quit {
            break;
        }
    }

    crossterm::execute!(std::io::stdout(), crossterm::event::DisableMouseCapture)?;
    ratatui::restore();

    Ok(())
}
