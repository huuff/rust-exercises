use std::str::FromStr;
use once_cell::sync::Lazy;
use regex::Regex;

pub enum TemperatureUnit {
    Celsius(i64),
    Fahrenheit(i64),
}

impl TemperatureUnit {
    pub fn name(&self) -> &'static str {
	match self {
	    TemperatureUnit::Celsius(_) => "celsius",
	    TemperatureUnit::Fahrenheit(_) => "fahrenheit",
	}
    }

    pub fn amount(&self) -> i64 {
	match self {
	    TemperatureUnit::Celsius(amount) => *amount,
	    TemperatureUnit::Fahrenheit(amount) => *amount,
	}
    }
}

// TODO: Test
#[derive(Debug, PartialEq, Eq)]
pub struct ParseTemperatureUnitError {}

 impl FromStr for TemperatureUnit {
    type Err = ParseTemperatureUnitError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
	static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?P<amount>\d+)(?P<unit>C|F)").expect("Wrong TemperatureUnit regex"));

	if let Some(captures) = RE.captures(s) {
	    // TODO: No unwrappin?
	    let amount = captures["amount"].parse::<i64>().unwrap();
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
