
use std::collections::HashSet;

use ratatui::widgets::TableState;

use crate::{Department, Employee};

pub enum Scene {
    DepartmentList {
	state: TableState,
    },
    DepartmentView {
	department: Department,
	employees: HashSet<Employee>,
	state: TableState,
    },
}


impl Scene {
    pub fn new_department_list() -> Self {
	Self::DepartmentList {
	    state: TableState::new().with_selected(Some(0)),
	} 
    }

    pub fn new_department_view(department: Department, employees: HashSet<Employee>) -> Self {
	Self::DepartmentView {
	    department,
	    employees,
	    state: TableState::new().with_selected(Some(0)),
	}
    }

    pub fn next(&mut self) {
	match self {
	    Scene::DepartmentList { state } => {
		state.select(match state.selected() {
		    Some(selected) => Some(if selected < Department::all().len()-1 { selected+1 } else { 0 }),
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
	    Scene::DepartmentList { state } => {
		state.select(match state.selected() {
		    Some(selected) => Some(if selected > 0 { selected-1 } else { Department::all().len()-1 }),
		    None => Some(0),
		})
	    }
	    Scene::DepartmentView { .. } => todo!(),
	}
    }
}
