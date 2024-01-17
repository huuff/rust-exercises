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
use scene::{Scene, DepartmentList, DepartmentView};
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

    pub fn select_employee(&mut self, department: &Department, employee: &Employee) {
	self.selected_employee = self.department_to_employees
	    .get_mut(department)
	    .unwrap()
	    .take(employee);
    }

}

// TODO: Some keybinding to cancel the employee move
fn main() -> anyhow::Result<()> {
    let mut app = App::new(create_sample_data());
    let mut scene = Scene::List(DepartmentList::new(&app.department_to_employees));
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
                    scene = match scene {
                        Scene::List( list_scene ) => {
			    match (list_scene.selected(), app.selected_employee.take()) {
				(Some(department), Some(employee)) => {
				    app.add_employee(&department, employee);
				    Scene::List(DepartmentList::new(&app.department_to_employees))
				}
				(Some(department), None) => {
				    let employees = &app.department_to_employees[&department];
				    Scene::View(DepartmentView::new(department, employees))
				}
				_ => Scene::List(list_scene)
			    }
                        }
                        Scene::View(view_scene) => {
                            // let employees =
                            //     app.department_to_employees.remove(&department).unwrap();
                            // let (employee, employees) =
                            //     extract(employees.into_iter().enumerate(), |(i, _)| {
                            //         state.selected().is_some_and(|selected| selected == *i)
                            //     });
                            // let employees = employees
                            //     .into_iter()
                            //     .map(|it| it.1)
                            //     .collect::<BTreeSet<Employee>>();
                            // app.department_to_employees.insert(department, employees);
                            // app.selected_employee = employee.map(|it| it.1);
			    if let Some(selected_employee) = view_scene.selected() {
				let department = view_scene.department;
				app.select_employee(&department, selected_employee);
			    }
			    Scene::List(DepartmentList::new(&app.department_to_employees))
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
