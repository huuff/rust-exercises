use std::str::FromStr;
use regex::Regex;

pub enum TemperatureUnit {
    Celsius(i64),
    Fahrenheit(i64),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseTemperatureUnitError {}

impl FromStr for TemperatureUnit {
    type Err = ParseTemperatureUnitError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
	let celsius_regex = Regex::new(r"(\d+)C").expect("Wrong celsius regex");
	let fahrenheit_regex = Regex::new(r"(\d+)F").expect("Wrong fahrenheit regex");

	if celsius_regex.is_match(s) {
	    Ok(Self::Celsius(0))
	} else if fahrenheit_regex.is_match(s) {
	    Ok(Self::Fahrenheit(0))
	} else {
	    Err(ParseTemperatureUnitError{}) 
	}

    }
}
