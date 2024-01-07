use ratatui::widgets::TableState;

pub enum Scene {
    DepartmentList(TableState),
    EmployeeList,
}


impl Scene {
    pub fn initial() -> Self {
	Self::DepartmentList(TableState::new().with_selected(Some(0)))
    }

    pub fn next(&mut self) {
	match self {
	    Scene::DepartmentList(table) => {
		table.select(match table.selected() {
		    Some(selected) => Some(selected + 1),
		    None => Some(0),
		})
	    }
	    Scene::EmployeeList => todo!(),
	}
    }

    pub fn previous(&mut self) {
	match self {
	    Scene::DepartmentList(table) => {
		table.select(match table.selected() {
		    Some(selected) => Some(selected - 1),
		    None => Some(0),
		})
	    }
	    Scene::EmployeeList => todo!(),
	}
    }
}
