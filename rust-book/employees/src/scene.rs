use ratatui::widgets::TableState;

use crate::{Department, util::Loopable, types::{EmployeeSet, DepartmentToEmployeeMap}};


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
	    state: TableState::new().with_selected(if departments_to_employees.len() != 0 { Some(0) } else { None }),
	} 
    }

    pub fn new_department_view(department: Department, employees: &'a EmployeeSet) -> Self {
	Self::DepartmentView {
	    department,
	    employees,
	    state: TableState::new().with_selected(if employees.len() != 0 { Some(0) } else { None }),
	}
    }

    pub fn next(&mut self) {
	let len = self.table_len();
	let state = self.state();
	state.select(state.selected().map(|it| it.next_in(0..len)));
    }

    pub fn previous(&mut self) {
	let len = self.table_len();
	let state = self.state();
	state.select(state.selected().map(|it| it.previous_in(0..len)));
    }

    fn table_len(&self) -> usize {
	match self {
	    Scene::DepartmentList { departments_to_employees, .. } => departments_to_employees.len(),
	    Scene::DepartmentView { employees, .. } => employees.len(),
	}
    }

    fn state(&mut self) -> &mut TableState {
	match self {
	    Scene::DepartmentList { state, ..} => state,
	    Scene::DepartmentView { state , .. } => state,
	}
    }
}
