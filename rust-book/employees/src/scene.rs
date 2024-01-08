

use ratatui::widgets::TableState;

use crate::{Department, types::{DepartmentToEmployeeMap, EmployeeSet}};


pub enum Scene {
    DepartmentList {
	department_to_employees: DepartmentToEmployeeMap,
	state: TableState,
    },
    DepartmentView {
	department: Department,
	employees: EmployeeSet,
	state: TableState,
    },
}


impl Scene {
    pub fn new_department_list(department_to_employees: DepartmentToEmployeeMap) -> Self {
	Self::DepartmentList {
	    department_to_employees,
	    state: TableState::new().with_selected(Some(0)),
	} 
    }

    pub fn new_department_view(department: Department, employees: EmployeeSet) -> Self {
	Self::DepartmentView {
	    department,
	    employees,
	    state: TableState::new().with_selected(Some(0)),
	}
    }

    pub fn next(&mut self) {
	match self {
	    Scene::DepartmentList { state, department_to_employees } => {
		state.select(match state.selected() {
		    Some(selected) => Some(if selected < department_to_employees.len()-1 { selected+1 } else { 0 }),
		    None => Some(0),
		})
	    }
	    Scene::DepartmentView { state, employees, .. } => {
		state.select(match state.selected() {
		    Some(selected) => Some(if selected < employees.len()-1 { selected+1 } else { 0 }),
		    None => Some(0),
		})
	    }
	}
    }

    pub fn previous(&mut self) {
	match self {
	    Scene::DepartmentList { state, department_to_employees } => {
		state.select(match state.selected() {
		    Some(selected) => Some(if selected > 0 { selected-1 } else { department_to_employees.len()-1 }),
		    None => Some(0),
		})
	    }
	    Scene::DepartmentView { .. } => todo!(),
	}
    }
}
