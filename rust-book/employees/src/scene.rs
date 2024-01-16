use ratatui::widgets::TableState;

use crate::{Department, util::Loopable, App, types::{EmployeeSet, DepartmentToEmployeeMap}};


// TODO: I added the employees to the DepartmentView... try to also add
// any necessary state to render it to DepartmentList
pub enum Scene<'a> {
    DepartmentList {
	state: TableState,
	departments_to_employees: &'a DepartmentToEmployeeMap,
    },
    DepartmentView {
	department: Department,
	employees: &'a EmployeeSet,
	state: TableState,
    },
}


impl<'a> Scene<'a> {
    pub fn new_department_list(departments_to_employees: &'a DepartmentToEmployeeMap) -> Self {
	Self::DepartmentList {
	    departments_to_employees,
	    state: TableState::new().with_selected(None),
	} 
    }

    pub fn new_department_view(department: Department, employees: &'a EmployeeSet) -> Self {
	Self::DepartmentView {
	    department,
	    employees,
	    state: TableState::new().with_selected(None),
	}
    }

    pub fn next(&mut self, app: &App) {
	match self {
	    Scene::DepartmentList { state, departments_to_employees, .. } => {
		state.select(match state.selected() {
		    Some(selected) => Some(selected.next_in(0..(departments_to_employees.len()-1))),
		    None => if app.num_departments() != 0 { Some(0) } else { None },
		})
	    }
	    Scene::DepartmentView { state, employees, .. } => {
		state.select(match state.selected() {
		    Some(selected) => Some(selected.next_in(0..(employees.len()-1))),
		    None => if employees.len() != 0 { Some(0) } else { None },
		})
	    }
	}
    }

    pub fn previous(&mut self, app: &App) {
	match self {
	    Scene::DepartmentList { state, departments_to_employees, .. } => {
		state.select(match state.selected() {
		    Some(selected) => Some(selected.previous_in(0..(departments_to_employees.len()-1))),
		    None => if app.num_departments() != 0 { Some(0) } else { None },
		})
	    }
	    Scene::DepartmentView { state, department, employees, .. } => {
		state.select(match state.selected() {
		    Some(selected) => Some(selected.previous_in(0..(employees.len()-1))),
		    None => if employees.len() != 0 { Some(0) } else { None },
		})
	    },
	}
    }
}
