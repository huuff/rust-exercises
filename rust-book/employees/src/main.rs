mod ui;
mod scene;
mod event;

use std::collections::{HashMap, HashSet};
use map_macro::hash_set;
use crossterm::event::KeyCode;
use event::{EventHandler, Event};
use scene::Scene;
use strum::{EnumIter, IntoStaticStr};


#[derive(PartialEq, Eq, Hash, EnumIter, IntoStaticStr, Clone, Copy, strum::Display)]
pub enum Department {
    Sales,
    Engineering,
    Marketing,
    Accounting,
    None,
}

const DEPARTMENTS: [Department; 5] = [Department::Sales, Department::Engineering, Department::Marketing, Department::Accounting, Department::None];

#[derive(PartialEq, Eq, Hash)]
pub struct Employee {
    name: String,
}

pub struct App {
    department_to_employees: HashMap<Department, HashSet<Employee>>,
    scene: Scene,
}

impl App {
    pub fn new() -> Self {
	Self {
	    department_to_employees: HashMap::new(),
	    scene: Scene::initial(),
 	}
    }
}

fn create_initial_employees() -> HashSet<Employee> {
    hash_set! {
	Employee { name: "Amir".to_string() },
	Employee { name: "Sally".to_string() },
    }
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
		    KeyCode::Char('q') => { break }
		    KeyCode::Down => {  app.scene.next() }
		    KeyCode::Up => { app.scene.previous() }
		    _ => {}
		}
	    }
	    _ => {}
	}
    }

    ui::close_terminal()?;
    Ok(())
}


