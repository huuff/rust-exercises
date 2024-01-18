use std::{hash::{Hasher, Hash}, cmp};

use once_cell::sync::Lazy;
use strum::{EnumIter, IntoStaticStr};


#[derive(PartialEq, Eq, Hash, EnumIter, IntoStaticStr, Clone, Copy, strum::Display, PartialOrd, Ord)]
pub enum Department {
    Sales,
    Engineering,
    Marketing,
    Accounting,
    None,
}

pub struct Employee {
    pub id: u32,
    pub name: String,
    pub salary: f64,
    pub department: Department,
}

impl Employee {
    pub fn new(name: String, salary: f64) -> Self {
	static NEXT_ID: Lazy<u32> = Lazy::new(|| 0);
	let employee = Employee {
	    id: *NEXT_ID,
	    name,
	    salary,
	    department: Department::None,
	};
	*NEXT_ID += 1;
	employee
    }
}


impl PartialEq for Employee {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Eq for Employee { }

impl PartialOrd for Employee {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
       self.name.partial_cmp(&other.name)
    }
}

impl Ord for Employee {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.partial_cmp(&other).unwrap()
    }
}
