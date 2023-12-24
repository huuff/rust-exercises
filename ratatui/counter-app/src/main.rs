use std::io;
use anyhow::Result;
use crossterm::{
    event::{self, Event::Key, KeyCode::Char},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::{CrosstermBackend, Terminal, Frame},
    widgets::Paragraph,
};

struct App {
    counter: i64,
    should_quit: bool,
}

impl App {
    fn new() -> Self {
	App { counter: 0, should_quit: false }
    }
}

fn main() -> Result<()> {
    startup()?;
    let status = run();
    shutdown()?;
    status?;
    Ok(())
}

fn run() -> Result<()> {
    let mut t = Terminal::new(CrosstermBackend::new(io::stderr()))?;

    let mut app = App::new();

    loop {
	t.draw(|f| ui(&app, f))?;

	update(&mut app)?;

	if app.should_quit {
	    break;
	}
    }

    Ok(())
}


fn startup() -> Result<()> {
    enable_raw_mode()?;
    execute!(io::stderr(), EnterAlternateScreen)?;
    Ok(())
}

fn shutdown() -> Result<()> {
    execute!(io::stderr(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn ui(app: &App, f: &mut Frame) {
    f.render_widget(Paragraph::new(format!("Counter: {}", app.counter)), f.size());
}

fn update(app: &mut App) -> Result<()> {
    if event::poll(std::time::Duration::from_millis(250))? {
	if let Key(key) = event::read()? {
	    if key.kind == event::KeyEventKind::Press {
		match key.code {
		    Char('j') => app.counter += 1,
		    Char('k') => app.counter -= 1,
		    Char('q') => app.should_quit = true,
		    _ => {},
		}
	    }
	}
    }


    Ok(())
}
