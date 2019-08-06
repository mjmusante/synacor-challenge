mod util;

use std::io;

use termion::event::Key;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Block, Borders};

use crate::util::event::{Event, Events};
// use tui::layout::{Layout, Constraint, Direction};

fn main() -> Result<(), failure::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.draw(|mut f| {
        let size = f.size();
        Block::default()
            .title("Block")
            .borders(Borders::ALL)
            .render(&mut f, size);
    })?;

    let events = Events::new();

    loop {

        match events.next()? {
            Event::Input(key) => match key {
                Key::Char('q') => {
                    break;
                },
                _ => { },
            },
            _ => { },

        }
    }

    Ok(())
}
