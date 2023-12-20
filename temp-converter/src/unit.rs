use std::str::FromStr;
use once_cell::sync::Lazy;
use regex::Regex;

pub enum TemperatureUnit {
    Celsius(f64),
    Fahrenheit(f64),
}

impl TemperatureUnit {
    // TODO: Rather than amount and name, implement display and limit precision so it's not ugly
    pub fn name(&self) -> &'static str {
	match self {
	    TemperatureUnit::Celsius(_) => "celsius",
	    TemperatureUnit::Fahrenheit(_) => "fahrenheit",
	}
    }

    pub fn amount(&self) -> f64 {
	match self {
	    TemperatureUnit::Celsius(amount) | TemperatureUnit::Fahrenheit(amount) => *amount
	}
    }

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

// TODO: Test
#[derive(Debug, PartialEq, Eq)]
pub struct ParseTemperatureUnitError {}

 impl FromStr for TemperatureUnit {
    type Err = ParseTemperatureUnitError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
	static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?P<amount>\d+\.?\d*)(?P<unit>C|F)").expect("Wrong TemperatureUnit regex"));

	// TODO: Can I make this nicer?
	if let Some(captures) = RE.captures(s) {
	    // TODO: No unwrappin?
	    let amount = captures["amount"].parse::<f64>().unwrap();
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
