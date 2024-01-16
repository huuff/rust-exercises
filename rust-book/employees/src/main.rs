mod data;
mod event;
mod models;
mod scene;
mod types;
mod ui;
mod util;

use std::collections::BTreeSet;

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

    pub fn add_employee(&mut self, department: &Department, employee: Employee) {
	self.department_to_employees
	    .get_mut(department)
	    .unwrap()
	    .insert(employee);
    }

}

// TODO: Some keybinding to cancel the employee move
fn main() -> anyhow::Result<()> {
    let mut app = App::new(create_sample_data());
    let mut scene = Scene::new_department_list(&app.department_to_employees);
    let event_handler = EventHandler::new(16);

    let mut terminal = ui::init_terminal()?;

    loop {
        match event_handler.next()? {
            Event::Tick => {
                terminal.draw(|frame| ui::render(frame, &mut scene, &app))?;
            }
            Event::Key(key) => match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Down => scene.next(),
                KeyCode::Up => scene.previous(),
                KeyCode::Enter => {
                    match scene {
                        Scene::DepartmentList { ref state, .. } => {
                            if let None = state.selected() {
                                continue;
                            };

                            let selected = state.selected().unwrap();
                            let department =
                                *app.department_to_employees.keys().nth(selected).unwrap();
                            scene = if let Some(selected_employee) = app.selected_employee.take() {
				app.add_employee(&department, selected_employee);
                                Scene::new_department_list(&app.department_to_employees)
                            } else {
                                Scene::new_department_view(
                                    department,
                                    &app.department_to_employees[&department],
                                )
                            };
                        }
                        Scene::DepartmentView {
                            department, state, ..
                        } => {
                            let employees =
                                app.department_to_employees.remove(&department).unwrap();
                            let (employee, employees) =
                                extract(employees.into_iter().enumerate(), |(i, _)| {
                                    state.selected().is_some_and(|selected| selected == *i)
                                });
                            let employees = employees
                                .into_iter()
                                .map(|it| it.1)
                                .collect::<BTreeSet<Employee>>();
                            app.department_to_employees.insert(department, employees);
                            app.selected_employee = employee.map(|it| it.1);
                            scene = Scene::new_department_list(&app.department_to_employees);
                        }
                    };
                }
                _ => {}
            },
            _ => {}
        }
    }

    ui::close_terminal()?;
    Ok(())
}
