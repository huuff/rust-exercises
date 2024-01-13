use ratatui::widgets::TableState;

use crate::{Department, util::Loopable, App};


pub enum Scene {
    DepartmentList {
	state: TableState,
    },
    DepartmentView {
	department: Department,
	state: TableState,
    },
}


impl Scene {
    pub fn new_department_list() -> Self {
	Self::DepartmentList {
	    state: TableState::new().with_selected(Some(0)),
	} 
    }

    pub fn new_department_view(department: Department) -> Self {
	Self::DepartmentView {
	    department,
	    state: TableState::new().with_selected(Some(0)),
	}
    }

    // TODO: Moving breaks terribly when the department is empty!
    pub fn next(&mut self, app: &App) {
	match self {
	    Scene::DepartmentList { state, .. } => {
		state.select(match state.selected() {
		    Some(selected) => Some(selected.next_in(0..(app.num_departments()-1))),
		    None => Some(0),
		})
	    }
	    Scene::DepartmentView { state, department, .. } => {
		state.select(match state.selected() {
		    Some(selected) => Some(selected.next_in(0..(app.num_employees(department)-1))),
		    None => Some(0),
		})
	    }
	}
    }

    pub fn previous(&mut self, app: &App) {
	match self {
	    Scene::DepartmentList { state, .. } => {
		state.select(match state.selected() {
		    Some(selected) => Some(selected.previous_in(0..(app.num_departments()-1))),
		    None => Some(0),
		})
	    }
	    Scene::DepartmentView { state, department, .. } => {
		state.select(match state.selected() {
		    Some(selected) => Some(selected.previous_in(0..(app.num_employees(department) - 1))),
		    None => Some(0),
		})
	    },
	}
    }
}
