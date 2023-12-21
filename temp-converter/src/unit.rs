use std::{str::FromStr, fmt::Display};
use once_cell::sync::Lazy;
use regex::Regex;

#[derive(PartialEq, Debug)]
pub enum TemperatureUnit {
    Celsius(f64),
    Fahrenheit(f64),
}

impl TemperatureUnit {

    // TODO Test both of these
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
	static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?P<amount>\d+\.?\d*)(?P<unit>C|F)")
					   .expect("Wrong TemperatureUnit regex"));

	let Some(captures) = RE.captures(s) else {
	    return Err(ParseTemperatureUnitError {})
	};
	
	
	let amount = captures["amount"].parse::<f64>()
	    .or_else(|_| Err(ParseTemperatureUnitError{}))?;
	match &captures["unit"] {
	    "C" => Ok(TemperatureUnit::Celsius(amount)),
	    "F" => Ok(TemperatureUnit::Fahrenheit(amount)),
	    _ => Err(ParseTemperatureUnitError {}),
	}

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn parse_celsius_integer() {
       let result = "3C".parse::<TemperatureUnit>(); 

	assert_eq!(result, Ok(TemperatureUnit::Celsius(3.0)));
    }

    #[test]
    fn parse_fahrenheit_decimal() {
	let result = "83.5F".parse::<TemperatureUnit>();

	assert_eq!(result, Ok(TemperatureUnit::Fahrenheit(83.5)));
    }

    #[test]
    fn parse_error() {
	let result = "lol".parse::<TemperatureUnit>();

	assert_eq!(result, Err(ParseTemperatureUnitError {  }))
    }
}
