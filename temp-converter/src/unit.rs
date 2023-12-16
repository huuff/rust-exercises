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
	// TODO: Use once_cell
	let regex = Regex::new(r"(?P<amount>\d+)(?P<unit>C|F)").expect("Wrong TemperatureUnit regex");

	if let Some(captures) = regex.captures(s) {
	    // TODO: No unwrappin?
	    let amount = (&captures["amount"]).parse::<i64>().unwrap();
	    match &captures["unit"] {
		"C" => Ok(TemperatureUnit::Celsius(amount)),
		"F" => Ok(TemperatureUnit::Fahrenheit(amount)),
		_ => Err(ParseTemperatureUnitError {}),
	    }
	} else {
	    Err(ParseTemperatureUnitError {})
	}

    }
}
