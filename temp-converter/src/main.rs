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

    let Some(input) = &args.get(1) else { crash(INPUT_ERROR_MESSAGE); };

    let Ok(unit) = input.parse::<TemperatureUnit>() else { crash(INPUT_ERROR_MESSAGE); };

    println!("You entered {} {}", unit.amount(), unit.name());

    let result = match unit {
        TemperatureUnit::Celsius(_) => unit.to_fahrenheit(),
        TemperatureUnit::Fahrenheit(_) => unit.to_celsius(),
    };

    println!("That equals {} {}", result.amount(), result.name());


    Ok(())
}

fn crash(message: &str) -> ! {
    eprintln!("{message}");
    process::exit(1);
}
