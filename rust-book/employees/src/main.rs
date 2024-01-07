mod ui;
mod event;

use std::collections::HashMap;
use crossterm::event::KeyCode;
use event::{EventHandler, Event};
use ratatui::widgets::TableState;
use strum::{EnumIter, IntoStaticStr};


#[derive(PartialEq, Eq, Hash, EnumIter, IntoStaticStr, Clone, Copy, strum::Display)]
pub enum Department {
    Sales,
    Engineering,
    Marketing,
    Accounting,
    None,
}

pub struct Employee {
    name: String,
}

pub struct App {
    department_to_employees: HashMap<Department, Vec<Employee>>,
    table_state: TableState,
}

impl App {
    pub fn new() -> Self {
	Self {
	    department_to_employees: HashMap::new(),
	    table_state: TableState::default().with_selected(Some(0)),
 	}
    }
}

fn create_initial_employees() -> Vec<Employee> {
    vec![
	Employee { name: "Amir".to_string() },
	Employee { name: "Sally".to_string() },
    ]
}

fn main() -> anyhow::Result<()> {
    let mut app = App::new();
    app.department_to_employees.insert(Department::None, create_initial_employees());
    let event_handler = EventHandler::new(16);

    let mut terminal = ui::init_terminal()?;

    loop {
	match event_handler.next()? {
	    Event::Tick => {
		terminal.draw(|frame| ui::render(frame, &mut app))?;
	    },
	    Event::Key(key) => {
		match key.code {
		    KeyCode::Char('q') => { break; }
		    KeyCode::Down => { app.table_state.next() }
		    KeyCode::Up => { app.table_state.previous() }
		    _ => {}
		}
	    }
	    _ => {}
	}
    }

    ui::close_terminal()?;
    Ok(())
}

// TODO: Extract this somewhere else
trait Stepper {
    fn next(&mut self);
    fn previous(&mut self);
}

impl Stepper for TableState {
    fn next(&mut self) {
        self.select(match self.selected() {
            Some(selected) => Some(selected + 1),
            None => Some(0),
        })
    }

    fn previous(&mut self) {
        self.select(match self.selected() {
            Some(selected) => Some(selected - 1),
            None => Some(0),
        })
    }
}

