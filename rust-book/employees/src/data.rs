use std::collections::HashMap;

use crate::Employee;

pub struct EmployeeDb {
    employees: HashMap<u32, Employee>,
}

impl EmployeeDb {
    pub fn new() -> Self {
	Self { employees: HashMap::new() }
    }

    pub fn bootstrap(&mut self) {
	for employee in INITIAL_EMPLOYEES {
	    self.insert(employee);
	}
    }

    pub fn insert(&mut self, employee: Employee) {
	self.employees[&employee.id] = employee;
    }


    pub fn find_all(&self) -> impl Iterator<Item=&Employee> {
	self.employees.values()
    }
}

static INITIAL_EMPLOYEES: Vec<Employee> = vec![
    Employee::new("Steven".to_string(), 24000.00),
    Employee::new("Neena".to_string(), 17000.00),
    Employee::new("Lex".to_string(), 17000.00),
    Employee::new("Alexander".to_string(), 9000.00),
    Employee::new("Bruce".to_string(), 6000.00),
    Employee::new("David".to_string(), 4800.00),
    Employee::new("Valli".to_string(), 4800.00),
    Employee::new("Diana".to_string(), 4200.00),
    Employee::new("Nancy".to_string(), 12000.00),
    Employee::new("Daniel".to_string(), 9000.00),
    Employee::new("John".to_string(), 8200.00),
    Employee::new("Ismael".to_string(), 7700.00),
    Employee::new("Jose Manuel".to_string(), 7800.00),
    Employee::new("Luis".to_string(), 6900.00),
    Employee::new("Den".to_string(), 11000.00),
    Employee::new("Alexander".to_string(), 3100.00),
    Employee::new("Shelli".to_string(), 2900.00),
    Employee::new("Sigal".to_string(), 2800.00),
    Employee::new("Guy".to_string(), 2600.00),
    Employee::new("Karen".to_string(), 2500.00),
    Employee::new("Matthew".to_string(), 8000.00),
    Employee::new("Adam".to_string(), 8200.00),
    Employee::new("Payam".to_string(), 7900.00),
    Employee::new("Shanta".to_string(), 6500.00),
    Employee::new("Irene".to_string(), 2700.00),
    Employee::new("John".to_string(), 14000.00),
    Employee::new("Karen".to_string(), 13500.00),
    Employee::new("Jonathon".to_string(), 8600.00),
    Employee::new("Jack".to_string(), 8400.00),
    Employee::new("Kimberely".to_string(), 7000.00),
    Employee::new("Charles".to_string(), 6200.00),
    Employee::new("Sarah".to_string(), 4000.00),
    Employee::new("Britney".to_string(), 3900.00),
    Employee::new("Jennifer".to_string(), 4400.00),
    Employee::new("Michael".to_string(), 13000.00),
    Employee::new("Pat".to_string(), 6000.00),
    Employee::new("Susan".to_string(), 6500.00),
    Employee::new("Hermann".to_string(), 10000.00),
    Employee::new("Shelley".to_string(), 12000.00),
    Employee::new("William".to_string(), 8300.00),
];
