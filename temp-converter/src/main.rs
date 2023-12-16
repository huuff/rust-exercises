mod unit;

use std::{
    env,
    error::Error,
    process,
};
use unit::TemperatureUnit;
use exitcode;
use regex::Regex;


fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let Some(input) = &args.get(1) else {
	eprintln!("Wrong number of arguments, exactly one is needed");
	process::exit(exitcode::DATAERR);
    };

    let Ok(unit) = input.parse::<TemperatureUnit>() else {
	eprintln!("Wrong number of arguments, exactly one is needed");
	process::exit(exitcode::DATAERR);
    };

    match unit {
	TemperatureUnit::Celsius(amount) => println!("You entered {amount} celsius"),
	TemperatureUnit::Fahrenheit(amount) => println!("You entered {amount} fahrenheit"),
    }

    Ok(())
    
}
