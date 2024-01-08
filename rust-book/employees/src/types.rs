use std::collections::{BTreeMap, BTreeSet};

use crate::models::{Employee, Department};


pub type EmployeeSet = BTreeSet<Employee>;
pub type DepartmentToEmployeeMap = BTreeMap<Department, EmployeeSet>;
