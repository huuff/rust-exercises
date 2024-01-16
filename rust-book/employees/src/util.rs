use std::ops;

pub trait Loopable<T: Ord> {
    fn next_in(self, range: ops::Range<T>) -> Self;
    fn previous_in(self, range: ops::Range<T>) -> Self;
}

impl Loopable<usize> for usize {
    fn next_in(self, range: ops::Range<usize>) -> Self {
	if self == range.end - 1 {
	   range.start
	} else {
	    self + 1
	}
    }

    fn previous_in(self, range: ops::Range<usize>) -> Self {
	if self == range.start {
	    range.end - 1
	} else {
	    self - 1
	}
    }
}


// TODO: Can I make this an extension on iterator?
pub fn extract<T: Sized>(iter: impl Iterator<Item=T>, f: impl Fn(&T) -> bool) -> (Option<T>, Vec<T>) {
    let mut extracted: Option<T> = None;
    let mut rest: Vec<T> = Vec::new();

    for item in iter {
	if extracted.is_none() && f(&item) {
	    extracted = Some(item)
	} else {
	    rest.push(item);
	}
    }

    (extracted, rest)
}

