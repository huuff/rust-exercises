
struct ChristmasDay {
    ordinal: &'static str,
    cardinal: &'static str,
    gift: &'static str,
}

impl ChristmasDay {
    fn print_gift(&self, is_last: bool) {
	if is_last {
	    println!("And {} {}.", self.cardinal.to_ascii_lowercase(), self.gift) 
	} else {
	    println!("{} {},", self.cardinal, self.gift);
	}
    }
}

static GIFTS: [ChristmasDay; 12] = [
    ChristmasDay {
	ordinal: "first",
	cardinal: "A",
	gift: "partridge in a pear tree",
    },
    ChristmasDay {
	ordinal: "second",
	cardinal: "Two",
	gift: "turtle doves",
    },
    ChristmasDay {
	ordinal: "third",
	cardinal: "Three",
	gift: "French hens",
    },
    ChristmasDay {
	ordinal: "fourth",
	cardinal: "Four",
	gift: "calling birds",
    },
    ChristmasDay {
	ordinal: "fifth",
	cardinal: "Five",
	gift: "gold rings",
    },
    ChristmasDay {
	ordinal: "sixth",
	cardinal: "Six",
	gift: "geese a-laying",
    },
    ChristmasDay {
	ordinal: "seventh",
	cardinal: "Seven",
	gift: "swans a-swimming",
    },
    ChristmasDay {
	ordinal: "eighth",
	cardinal: "Eight",
	gift: "maids a-milking",
    },
    ChristmasDay {
	ordinal: "ninth",
	cardinal: "Nine",
	gift: "ladies dancing",
    },
    ChristmasDay {
	ordinal: "tenth",
	cardinal: "Ten",
	gift: "lords a-leaping",
    },
    ChristmasDay {
	ordinal: "eleventh",
	cardinal: "Eleven",
	gift: "pipers dancing",
    },
    ChristmasDay {
	ordinal: "twelfth",
	cardinal: "Twelve",
	gift: "drummers drumming",
    }
];

fn main() {
    for day in 1..=12 {
	let index = day-1;
	let day_struct = &GIFTS[index];

	println!("On the {} day of Chrismas my true love sent to me", day_struct.ordinal);
	day_struct.print_gift(false);

	for previous_day in (1..=(day-1)).rev() {
	    let index = previous_day-1;
	    let previous_day_struct = &GIFTS[index];
	    let is_last = previous_day == 1 && day != 1;

	    previous_day_struct.print_gift(is_last);
	}

	println!();

    }
}
