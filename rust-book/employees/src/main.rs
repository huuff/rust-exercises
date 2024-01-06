use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
pub enum Department {
    Sales,
    Engineering,
    Marketing,
    Accounting,
    None,
}

pub struct Employee {
    name: String,
}

#[derive(Default)]
struct App {
    department_to_employees: HashMap<Department, Vec<Employee>>
}

fn create_initial_employees() -> Vec<Employee> {
    vec![
	Employee { name: "Amir".to_string() },
	Employee { name: "Sally".to_string() },
    ]
}

fn main() {
    let mut app = App::default();
    // app.department_to_employees[&Department::None] = create_initial_employees();
    app.department_to_employees.insert(Department::None, create_initial_employees());
}
