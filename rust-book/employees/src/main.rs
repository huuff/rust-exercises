mod event;
mod data;
mod scene;
mod ui;
mod models;
mod types;
mod util;

use crate::models::{Department, Employee};
use crossterm::event::KeyCode;
use data::create_sample_data;
use event::{Event, EventHandler};
use scene::Scene;
use types::DepartmentToEmployeeMap;
use util::extract;


pub struct App {
    department_to_employees: DepartmentToEmployeeMap,
    selected_employee: Option<Employee>,
}

impl App {
    pub fn new(initial_staff: DepartmentToEmployeeMap) -> Self {
        Self {
	    department_to_employees: initial_staff,
	    selected_employee: None,
        }
    }

    pub fn num_departments(&self) -> usize {
	self.department_to_employees.len()
    }

    pub fn num_employees(&self, department: &Department) -> usize {
	self.department_to_employees[department].len()
    }
}

// TODO: Show a legend with keybindings
fn main() -> anyhow::Result<()> {
    let mut app = App::new(create_sample_data());
    let mut scene = Scene::new_department_list();
    let event_handler = EventHandler::new(16);

    let mut terminal = ui::init_terminal()?;

    loop {
        match event_handler.next()? {
            Event::Tick => {
                terminal.draw(|frame| ui::render(frame, &mut scene, &app))?;
            }
            Event::Key(key) => {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Down => scene.next(&app),
                    KeyCode::Up => scene.previous(&app),
                    KeyCode::Enter => {
                        scene = match scene {
                            Scene::DepartmentList { state, .. } => {
				// TODO: Maybe avoid the news?
                                if let Some(selected) = state.selected() {
                                    let department = *app.department_to_employees.keys().nth(selected).unwrap();
				    if let Some(selected_employee) = app.selected_employee.take() {
					app.department_to_employees.get_mut(&department).unwrap().insert(selected_employee);
					Scene::new_department_list()
				    } else {
					Scene::new_department_view(department)
				    }
                                } else {
				    Scene::new_department_list() // TODO: Show some message that some must be selected otherwise
				}
                            }
                            Scene::DepartmentView { department, state, ..  } => {
				let employees = app.department_to_employees.remove(&department).unwrap();
				let (employee, employees) = extract(employees.into_iter()
								    .enumerate(),
								    |(i, _)| state.selected().is_some_and(|selected| selected == *i) )
				    ;
				let (employee, employees) = (employee.unwrap().1, employees.into_iter().map(|it| it.1));
				app.department_to_employees.insert(department, employees.collect());
				app.selected_employee = Some(employee);
				Scene::new_department_list()
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
