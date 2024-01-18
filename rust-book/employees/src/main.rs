mod data;
mod event;
mod models;
mod scene;
mod ui;
mod util;
mod types;

use crate::models::{Department, Employee};
use crossterm::event::KeyCode;
use data::EmployeeDb;
use event::{Event, EventHandler};
use scene::{Scene, DepartmentList, DepartmentView};

pub struct App {
    employee_db: EmployeeDb,
    selected_employee: Option<Employee>,
}

impl App {
    pub fn new() -> Self {
        Self {
	    employee_db: EmployeeDb::new(),
            selected_employee: None,
        }
    }
}

// TODO: Some keybinding to cancel the employee move
fn main() -> anyhow::Result<()> {
    let mut app = App::new();
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
				    //app.add_employee(&department, employee);
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
