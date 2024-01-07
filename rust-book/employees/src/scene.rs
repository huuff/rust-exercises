use ratatui::widgets::TableState;

pub enum Scene {
    DepartmentList(TableState),
    EmployeeList,
}


impl Scene {
    pub fn initial() -> Self {
	Self::DepartmentList(TableState::new().with_selected(Some(0)))
    }

    pub fn next(&mut self, max: usize) {
	match self {
	    Scene::DepartmentList(table) => {
		table.select(match table.selected() {
		    Some(selected) => Some(if selected < max { selected+1 } else { 0 }),
		    None => Some(0),
		})
	    }
	    Scene::EmployeeList => todo!(),
	}
    }

    pub fn previous(&mut self, max: usize) {
	match self {
	    Scene::DepartmentList(table) => {
		table.select(match table.selected() {
		    Some(selected) => Some(if selected > 0 { selected-1 } else { max }),
		    None => Some(0),
		})
	    }
	    Scene::EmployeeList => todo!(),
	}
    }
}
