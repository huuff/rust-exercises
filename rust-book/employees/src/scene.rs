use std::collections::HashMap;
use itertools::Itertools as _;

use ratatui::widgets::TableState;

use crate::{Department, util::Loopable, models::Employee, data::EmployeeDb};

pub struct DepartmentList {
    pub state: TableState,
    pub departments_to_employees: HashMap<Department, u32>,
}

impl DepartmentList {
    pub fn new(employee_db: &EmployeeDb) -> Self {
	let departments_to_employees = employee_db.find_all()
	    .group_by(|e| e.department)
	    ;
	DepartmentList {
	    departments_to_employees,
	    state: TableState::new().with_selected(if !departments_to_employees.is_empty() { Some(0) } else { None }),
	} 
    }
}

pub struct DepartmentView<'a, T: Iterator<Item=&'a Employee>> {
    pub department: Department,
    pub employees: T,
    pub state: TableState,
}

impl<'a, T: Iterator<Item=&'a Employee>> DepartmentView<'a, T> {
    pub fn new(department: Department, employees: T) -> Self {
	DepartmentView {
	    department,
	    employees,
	    state: TableState::new().with_selected(if !employees.is_empty() { Some(0) } else { None }),
	}
    }
}


pub enum Scene<'a, T: Iterator<Item=&'a Employee>> {
    List(DepartmentList),
    View(DepartmentView<'a, T>),
}

impl<'a, T: Iterator<Item=&'a Employee>> Scene<'a, T> {
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
