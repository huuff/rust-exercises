use std::{str::FromStr, fmt::Display};
use once_cell::sync::Lazy;
use regex::Regex;

pub enum TemperatureUnit {
    Celsius(f64),
    Fahrenheit(f64),
}

impl TemperatureUnit {

    pub fn to_celsius(self) -> TemperatureUnit {
	match self {
	    TemperatureUnit::Celsius(_) => self,
	    TemperatureUnit::Fahrenheit(amount) => {
		TemperatureUnit::Celsius((amount - 32.0) * (5.0/9.0))
	    }
	}
    }

    pub fn to_fahrenheit(self) -> TemperatureUnit {
	match self {
	    TemperatureUnit::Celsius(amount) => {
		TemperatureUnit::Fahrenheit((amount * 5.0/9.0) + 32.0)
	    },
	    TemperatureUnit::Fahrenheit(_) => self,
	}
    }
}

impl Display for TemperatureUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	match self {
	    TemperatureUnit::Celsius(amount) => write!(f, "{:.2}C", amount),
	    TemperatureUnit::Fahrenheit(amount) => write!(f, "{:.2}F", amount),
	}
    }
}

// TODO: Test
#[derive(Debug, PartialEq, Eq)]
pub struct ParseTemperatureUnitError {}

 impl FromStr for TemperatureUnit {
    type Err = ParseTemperatureUnitError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
	static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?P<amount>\d+\.?\d*)(?P<unit>C|F)").expect("Wrong TemperatureUnit regex"));

	// TODO: Can I make this nicer?
	if let Some(captures) = RE.captures(s) {
	    let amount = captures["amount"].parse::<f64>()
		.or_else(|_| Err(ParseTemperatureUnitError{}))?;
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
