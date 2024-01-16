use ratatui::widgets::TableState;

use crate::{Department, util::Loopable, types::{EmployeeSet, DepartmentToEmployeeMap}};

pub struct DepartmentList<'a> {
    pub state: TableState,
    pub departments_to_employees: &'a DepartmentToEmployeeMap,
}

impl<'a> DepartmentList<'a> {
    pub fn new(departments_to_employees: &'a DepartmentToEmployeeMap) -> Self {
	DepartmentList {
	    departments_to_employees,
	    state: TableState::new().with_selected(if !departments_to_employees.is_empty() { Some(0) } else { None }),
	} 
    }

    pub fn selected(&self) -> Option<Department> {
	self.state.selected().map(|selected| {
	    *self.departments_to_employees.keys().nth(selected).unwrap()
	})
    }
}

pub struct DepartmentView<'a> {
    pub department: Department,
    pub employees: &'a EmployeeSet,
    pub state: TableState,
}

impl<'a> DepartmentView<'a> {
    pub fn new(department: Department, employees: &'a EmployeeSet) -> Self {
	DepartmentView {
	    department,
	    employees,
	    state: TableState::new().with_selected(if !employees.is_empty() { Some(0) } else { None }),
	}
    }
}


pub enum Scene<'a> {
    List(DepartmentList<'a>),
    View(DepartmentView<'a>),
}

impl<'a> Scene<'a> {
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
	    Scene::List(DepartmentList { departments_to_employees, ..  }) => departments_to_employees.len(),
	    Scene::View(DepartmentView { employees, .. }) => employees.len(),
	}
    }

    fn state(&mut self) -> &mut TableState {
	match self {
	    Scene::List(DepartmentList { state, .. }) => state,
	    Scene::View(DepartmentView { state , .. }) => state,
	}
    }
}
