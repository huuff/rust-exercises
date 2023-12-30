mod app;
mod event;
mod util;

use std::io;

use crossterm::{
    event::KeyCode,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use event::{Event, EventHandler};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, SegmentSize},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal, TerminalOptions, Viewport,
};
use util::centered_rect;

fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::with_options(
        CrosstermBackend::new(io::stdout()),
        TerminalOptions {
            viewport: Viewport::Fullscreen,
        },
    )?;

    let event_handler = EventHandler::new(16);

    loop {
        match event_handler.next()? {
            Event::Tick => {
                terminal.draw(ui)?;
            }
            Event::Key(key) => {
                match key.code {
                    KeyCode::Char('q') => {
                        break;
                    }
                    _ => {}
                };
            }
            _ => {}
        }
    }

    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn ui(f: &mut Frame) {
    let vertical_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Min(0),
            Constraint::Length(3),
            Constraint::Min(0),
        ],
    )
    .segment_size(SegmentSize::EvenDistribution);

    let horizontal_layout = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Min(0),
            Constraint::Length(20),
            Constraint::Min(0),
        ],
    )
    .segment_size(SegmentSize::EvenDistribution);

    let middle_rect = horizontal_layout.split(vertical_layout.split(f.size())[1])[1];
    
    let input_block = Block::default()
        .borders(Borders::ALL)
        .title("Enter your guess");
    f.render_widget(input_block, middle_rect);
}
