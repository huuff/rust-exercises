use std::{
    env,
    error::Error,
    process,
};
use exitcode;
use regex::Regex;

enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
	eprintln!("Wrong number of arguments, exactly one is needed");
	process::exit(exitcode::DATAERR);
    }

    let input = &args[0];

    let celsius_regex = Regex::new(r"(\d+)C")?;
    let fahrenheit_regex = Regex::new(r"(\d+)F")?;


    let unit: Option<TemperatureUnit> = if celsius_regex.is_match(&input) {
	Some(TemperatureUnit::Celsius)
    } else if fahrenheit_regex.is_match(&input) {
	Some(TemperatureUnit::Fahrenheit)
    } else {
	None
    };

    Ok(())
    
}
