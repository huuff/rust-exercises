use std::{collections::HashMap, fmt::Display};

use itertools::Itertools;

pub struct Stats {
    pub median: f64,
    pub mode: Vec<u32>,
}

impl Stats {
    pub fn from(input: &Vec<u32>) -> Self {
	Self {
	    median: calculate_median(&input),
	    mode: calculate_modes(&input),
	}
    }
}

impl Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	write!(f, "Median: {}, Mode: {:?}", self.median, self.mode)
    }
}

fn calculate_median(input: &Vec<u32>) -> f64 {
    let sorted = input.into_iter()
        .map(|it| it.to_owned())
        .sorted()
        .collect::<Vec<u32>>();

    if sorted.len() % 2 != 0 {
	sorted[sorted.len()/2] as f64
    } else {
	let midpoint = sorted.len()/2;
	(sorted[midpoint-1] + sorted[midpoint]) as f64 / 2_f64
    }
}

fn calculate_modes(input: &Vec<u32>) -> Vec<u32> {
    let mut occurrences: HashMap<u32, u32> = HashMap::new();

    for n in input {
	let n_occurrences = occurrences.entry(*n).or_insert(0);
	*n_occurrences += 1;
    }

    occurrences.into_iter()
	.max_set_by_key(|it| (*it).1)
	.into_iter()
	.map(|it| it.0)
	.collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use assertor::*;
    
    #[test]
    fn odd_median() {
       let input = vec![3, 5, 7, 2, 1]; 

	assert_eq!(calculate_median(&input), 3_f64)
    }

    #[test]
    fn even_median() {
       let input = vec![3, 5, 2, 1]; 

	assert_eq!(calculate_median(&input), 2.5)
    }

    #[test]
    fn finds_mode() {
	let input = vec![1, 2, 3, 2, 5, 3, 2];

	assert_eq!(calculate_modes(&input), vec![2]);
    }

    #[test]
    fn doesnt_find_mode() {
	let input = vec![];

	assert_eq!(calculate_modes(&input), vec![]);
    }

    #[test]
    fn finds_many_modes() {
	let input = vec![1, 2, 3, 4, 5];

	assert_that!(calculate_modes(&input)).contains_exactly(&input)
    }
}
