mod event;

use event::{Event, EventHandler};
use std::io;
use time::{Duration, Instant};

use crossterm::{
    event::KeyCode,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::Alignment,
    widgets::{block::Position, Block, Borders, Paragraph},
    Frame, Terminal, TerminalOptions, Viewport,
};

fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut terminal = Terminal::with_options(
        CrosstermBackend::new(io::stdout()),
        TerminalOptions {
            viewport: Viewport::Inline(5),
        },
    )?;

    let start_time = Instant::now();
    let event_handler = EventHandler::new(250);
    terminal.draw(|f| ui(f, start_time.elapsed()))?;

    loop {
        match event_handler.next()? {
            Event::Tick => {
                terminal.draw(|f| ui(f, start_time.elapsed()))?;
            }
            Event::Key(key_event) => match key_event.code {
                KeyCode::Char('q') => {
                    break;
                }
                _ => {}
            },
            _ => {}
        };
    }

    disable_raw_mode()?;

    Ok(())
}

fn ui(f: &mut Frame, elapsed_time: Duration) {
    let p = Paragraph::new(format!("Hello World. {}s", elapsed_time.whole_seconds())).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Press q to exit")
            .title_position(Position::Bottom)
            .title_alignment(Alignment::Right),
    );

    f.render_widget(p, f.size());
}
