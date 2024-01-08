mod event;
mod scene;
mod ui;

use core::hash::Hash;
use crossterm::event::KeyCode;
use event::{Event, EventHandler};
use map_macro::hash_set;
use scene::Scene;
use std::{collections::{HashMap, HashSet}, hash::Hasher};
use strum::{EnumIter, IntoStaticStr};

// TODO: Extract employee and department stuff somewhere else
#[derive(PartialEq, Eq, Hash, EnumIter, IntoStaticStr, Clone, Copy, strum::Display)]
pub enum Department {
    Sales,
    Engineering,
    Marketing,
    Accounting,
    None,
}

const DEPARTMENTS: [Department; 5] = [
    Department::Sales,
    Department::Engineering,
    Department::Marketing,
    Department::Accounting,
    Department::None,
];

impl Department {
    pub fn all() -> &'static [Department; 5] {
        &DEPARTMENTS
    }
}

pub struct Employee {
    name: String,
    salary: f64,
}

impl Hash for Employee {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}

impl PartialEq for Employee {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Eq for Employee { }

pub struct App {
    department_to_employees: HashMap<Department, HashSet<Employee>>,
    scene: Scene,
}

impl App {
    pub fn new() -> Self {
        Self {
            department_to_employees: HashMap::new(),
            scene: Scene::new_department_list(),
        }
    }
}

fn create_initial_employees() -> HashSet<Employee> {
    hash_set! {
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
    app.department_to_employees
        .insert(Department::None, create_initial_employees());
    let event_handler = EventHandler::new(16);

    let mut terminal = ui::init_terminal()?;

    loop {
        match event_handler.next()? {
            Event::Tick => {
                terminal.draw(|frame| ui::render(frame, &mut app))?;
            }
            Event::Key(key) => {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Down => app.scene.next(),
                    KeyCode::Up => app.scene.previous(),
                    KeyCode::Enter => {
                        match &app.scene {
                            Scene::DepartmentList { state } => {
                                if let Some(selected) = state.selected() {
                                    let department = Department::all()[selected];
				    let employees = app.department_to_employees.remove(&department).unwrap();
                                    app.scene = Scene::new_department_view(department, employees);
                                } // TODO: Show some message that some must be selected otherwise
                            }
                            Scene::DepartmentView { .. } => todo!(),
                        }
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
