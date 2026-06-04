mod app;
mod tui;

use crossterm::event::{self, Event, KeyEventKind};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = tui::init()?;
    let mut app = app::App::new();

    while !app.should_quit {
        terminal.draw(|f| app.render(f))?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    app.handle_key(key.code);
                }
            }
        }

        app.update();
    }

    tui::restore()?;
    Ok(())
}
