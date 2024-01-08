use std::{hash::{Hasher, Hash}, cmp};

use strum::{EnumIter, IntoStaticStr};


#[derive(PartialEq, Eq, Hash, EnumIter, IntoStaticStr, Clone, Copy, strum::Display)]
pub enum Department {
    Sales,
    Engineering,
    Marketing,
    Accounting,
    None,
}

const DEPARTMENTS: [Department; 5] = [
    Department::Sales,
    Department::Engineering,
    Department::Marketing,
    Department::Accounting,
    Department::None,
];

impl Department {
    pub fn all() -> &'static [Department; 5] {
        &DEPARTMENTS
    }
}

pub struct Employee {
    pub name: String,
    pub salary: f64,
}

impl Hash for Employee {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state)
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
