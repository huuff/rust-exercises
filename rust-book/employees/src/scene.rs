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
	    state: TableState::new().with_selected(None),
	} 
    }

    pub fn new_department_view(department: Department) -> Self {
	Self::DepartmentView {
	    department,
	    state: TableState::new().with_selected(None),
	}
    }

    pub fn next(&mut self, app: &App) {
	match self {
	    Scene::DepartmentList { state, .. } => {
		state.select(match state.selected() {
		    Some(selected) => Some(selected.next_in(0..(app.num_departments()-1))),
		    None => if app.num_departments() != 0 { Some(0) } else { None },
		})
	    }
	    Scene::DepartmentView { state, department, .. } => {
		state.select(match state.selected() {
		    Some(selected) => Some(selected.next_in(0..(app.num_employees(department)-1))),
		    None => if app.num_employees(department) != 0 { Some(0) } else { None },
		})
	    }
	}
    }

    pub fn previous(&mut self, app: &App) {
	match self {
	    Scene::DepartmentList { state, .. } => {
		state.select(match state.selected() {
		    Some(selected) => Some(selected.previous_in(0..(app.num_departments()-1))),
		    None => if app.num_departments() != 0 { Some(0) } else { None },
		})
	    }
	    Scene::DepartmentView { state, department, .. } => {
		state.select(match state.selected() {
		    Some(selected) => Some(selected.previous_in(0..(app.num_employees(department) - 1))),
		    None => if app.num_employees(department) != 0 { Some(0) } else { None },
		})
	    },
	}
    }
}
