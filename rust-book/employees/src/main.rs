mod event;
mod scene;
mod ui;
mod models;

use crate::models::{Department, Employee};
use crossterm::event::KeyCode;
use event::{Event, EventHandler};
use scene::{Scene, EmployeeSet};
use std::collections::HashMap;
use map_macro::{btree_set, hash_map};


pub struct App {
    department_to_employees: Option<HashMap<Department, EmployeeSet>>,
    scene: Scene,
}

impl App {
    pub fn new() -> Self {
	let department_to_employees = hash_map! {
	    Department::Accounting => EmployeeSet::new(),
	    Department::Engineering => EmployeeSet::new(),
	    Department::Marketing => EmployeeSet::new(),
	    Department::Sales => EmployeeSet::new(),
	    Department::None => create_initial_employees(),
	};
        Self {
            department_to_employees: None,
            scene: Scene::new_department_list(department_to_employees),
        }
    }
}

fn create_initial_employees() -> EmployeeSet {
    btree_set! {
	Employee { name: "Steven".to_string(), salary: 24000.00 },
	Employee { name: "Neena".to_string(), salary: 17000.00 },
	Employee { name: "Lex".to_string(), salary: 17000.00 },
	Employee { name: "Alexander".to_string(), salary: 9000.00 },
	Employee { name: "Bruce".to_string(), salary: 6000.00 },
	Employee { name: "David".to_string(), salary: 4800.00 },
	Employee { name: "Valli".to_string(), salary: 4800.00 },
	Employee { name: "Diana".to_string(), salary: 4200.00 },
	Employee { name: "Nancy".to_string(), salary: 12000.00 },
	Employee { name: "Daniel".to_string(), salary: 9000.00 },
	Employee { name: "John".to_string(), salary: 8200.00 },
	Employee { name: "Ismael".to_string(), salary: 7700.00 },
	Employee { name: "Jose Manuel".to_string(), salary: 7800.00 },
	Employee { name: "Luis".to_string(), salary: 6900.00 },
	Employee { name: "Den".to_string(), salary: 11000.00 },
	Employee { name: "Alexander".to_string(), salary: 3100.00 },
	Employee { name: "Shelli".to_string(), salary: 2900.00 },
	Employee { name: "Sigal".to_string(), salary: 2800.00 },
	Employee { name: "Guy".to_string(), salary: 2600.00 },
	Employee { name: "Karen".to_string(), salary: 2500.00 },
	Employee { name: "Matthew".to_string(), salary: 8000.00 },
	Employee { name: "Adam".to_string(), salary: 8200.00 },
	Employee { name: "Payam".to_string(), salary: 7900.00 },
	Employee { name: "Shanta".to_string(), salary: 6500.00 },
	Employee { name: "Irene".to_string(), salary: 2700.00 },
	Employee { name: "John".to_string(), salary: 14000.00 },
	Employee { name: "Karen".to_string(), salary: 13500.00 },
	Employee { name: "Jonathon".to_string(), salary: 8600.00 },
	Employee { name: "Jack".to_string(), salary: 8400.00 },
	Employee { name: "Kimberely".to_string(), salary: 7000.00 },
	Employee { name: "Charles".to_string(), salary: 6200.00 },
	Employee { name: "Sarah".to_string(), salary: 4000.00 },
	Employee { name: "Britney".to_string(), salary: 3900.00 },
	Employee { name: "Jennifer".to_string(), salary: 4400.00 },
	Employee { name: "Michael".to_string(), salary: 13000.00 },
	Employee { name: "Pat".to_string(), salary: 6000.00 },
	Employee { name: "Susan".to_string(), salary: 6500.00 },
	Employee { name: "Hermann".to_string(), salary: 10000.00 },
	Employee { name: "Shelley".to_string(), salary: 12000.00 },
	Employee { name: "William".to_string(), salary: 8300.00 },
    }
}

fn main() -> anyhow::Result<()> {
    let mut app = App::new();
    let event_handler = EventHandler::new(16);

    let mut terminal = ui::init_terminal()?;

    loop {
        match event_handler.next()? {
            Event::Tick => {
                terminal.draw(|frame| ui::render(frame, &mut app.scene))?;
            }
            Event::Key(key) => {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Down => app.scene.next(),
                    KeyCode::Up => app.scene.previous(),
                    KeyCode::Enter => {
                        app.scene = match app.scene {
                            Scene::DepartmentList { state, mut department_to_employees } => {
                                if let Some(selected) = state.selected() {
                                    let department = Department::all()[selected];
				    let employees = (department_to_employees).remove(&department).unwrap();
				    app.department_to_employees = Some(department_to_employees);
                                    Scene::new_department_view(department, employees)
                                } else {
				    Scene::DepartmentList { state, department_to_employees }
				} // TODO: Show some message that some must be selected otherwise
                            }
                            Scene::DepartmentView { department, employees, ..  } => {
				let mut department_to_employees = app.department_to_employees.take().unwrap();
				department_to_employees.insert(department, employees);
				Scene::new_department_list(department_to_employees)
			    }
                        };
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    ui::close_terminal()?;
    Ok(())
}
