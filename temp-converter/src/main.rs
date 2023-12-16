mod unit;

use std::{
    env,
    error::Error,
    process,
};
use unit::TemperatureUnit;

const INPUT_ERROR_MESSAGE: &str = "You must provide an input amount and unit. Examples: 32Cº, 45F";

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let Some(input) = &args.get(1) else {
	crash(INPUT_ERROR_MESSAGE);
    };

    let Ok(unit) = input.parse::<TemperatureUnit>() else {
	crash(INPUT_ERROR_MESSAGE);
    };

    println!("You entered {} {}", unit.amount(), unit.name());

    Ok(())
}

fn crash(message: &str) -> ! {
    eprintln!("{message}");
    process::exit(1);
}
