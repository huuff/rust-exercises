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
