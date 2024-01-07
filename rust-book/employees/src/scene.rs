
use ratatui::widgets::TableState;

use crate::Department;

pub enum Scene {
    DepartmentList {
	state: TableState,
    },
    DepartmentView {
	department: Department,
    },
}


impl Scene {
    pub fn initial() -> Self {
	Self::DepartmentList {
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
	    Scene::DepartmentView { department } => todo!(),
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
	    Scene::DepartmentView {department } => todo!(),
	}
    }
}
