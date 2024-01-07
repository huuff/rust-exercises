use std::collections::HashSet;

use ratatui::widgets::TableState;

use crate::{Employee, DEPARTMENTS};

pub enum Scene {
    DepartmentList {
	state: TableState,
	max: usize,
    },
    EmployeeList {
	employees: HashSet<Employee>,
    },
}


impl Scene {
    pub fn initial() -> Self {
	Self::DepartmentList {
	    state: TableState::new().with_selected(Some(0)),
	    max: DEPARTMENTS.len() - 1, // TODO: Or maybe just put departments here?
	} 
    }

    pub fn next(&mut self) {
	match self {
	    Scene::DepartmentList { state, max  } => {
		state.select(match state.selected() {
		    Some(selected) => Some(if selected < *max { selected+1 } else { 0 }),
		    None => Some(0),
		})
	    }
	    Scene::EmployeeList { employees } => todo!(),
	}
    }

    pub fn previous(&mut self) {
	match self {
	    Scene::DepartmentList { state, max } => {
		state.select(match state.selected() {
		    Some(selected) => Some(if selected > 0 { selected-1 } else { *max }),
		    None => Some(0),
		})
	    }
	    Scene::EmployeeList { employees } => todo!(),
	}
    }
}
