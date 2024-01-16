use ratatui::widgets::TableState;

use crate::{Department, util::Loopable, App, types::{EmployeeSet, DepartmentToEmployeeMap}};


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

    pub fn next(&mut self) {
	match self {
	    Scene::DepartmentList { state, departments_to_employees, .. } => {
		let num_departments = departments_to_employees.len();
		state.select(match state.selected() {
		    Some(selected) => Some(selected.next_in(0..num_departments)),
		    None => if num_departments != 0 { Some(0) } else { None },
		})
	    }
	    Scene::DepartmentView { state, employees, .. } => {
		let num_employees = employees.len();
		state.select(match state.selected() {
		    Some(selected) => Some(selected.next_in(0..num_employees)),
		    None => if num_employees != 0 { Some(0) } else { None },
		})
	    }
	}
    }

    pub fn previous(&mut self) {
	match self {
	    Scene::DepartmentList { state, departments_to_employees, .. } => {
		let num_departments = departments_to_employees.len();
		state.select(match state.selected() {
		    Some(selected) => Some(selected.previous_in(0..num_departments)),
		    None => if num_departments != 0 { Some(0) } else { None },
		})
	    }
	    Scene::DepartmentView { state, employees, .. } => {
		let num_employees = employees.len();
		state.select(match state.selected() {
		    Some(selected) => Some(selected.previous_in(0..num_employees)),
		    None => if num_employees != 0 { Some(0) } else { None },
		})
	    },
	}
    }
}
