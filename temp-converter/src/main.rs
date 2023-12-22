mod unit;

use std::{
    env,
    error::Error,
    process, io::Write,
};
use unit::{TemperatureUnit, ParseTemperatureUnitError};

const INPUT_ERROR_MESSAGE: &str = "You must provide an input amount and unit. Examples: 32Cº, 45F";

fn main() -> Result<(), Box<dyn Error>> {
    let Ok(input) = get_input() else { crash(INPUT_ERROR_MESSAGE); };

    println!("You entered {input}");

    let result = match input {
        TemperatureUnit::Celsius(_) => input.to_fahrenheit(),
        TemperatureUnit::Fahrenheit(_) => input.to_celsius(),
    };

    println!("That equals {result}");


    Ok(())
}

fn get_input() -> Result<TemperatureUnit, ParseTemperatureUnitError> {
    let args: Vec<String> = env::args().collect();

    if let Some(input) = &args.get(1) {
	input.parse::<TemperatureUnit>()
    } else {
	let mut input = String::new();

	loop {
	    print!("Enter your input unit (e.g. 80F or 35C): ");
	    let _ = std::io::stdout().flush();
	    if std::io::stdin().read_line(&mut input).is_ok() {
		break;
	    } else {
		eprint!("Error processing input, please try again");
	    }
	}

	input.parse::<TemperatureUnit>()
    }
}

fn crash(message: &str) -> ! {
    eprintln!("{message}");
    process::exit(1);
}
