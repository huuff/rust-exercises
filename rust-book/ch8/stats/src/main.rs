use std::{env, error::Error};

mod stats;


fn main() -> Result<(), Box<dyn Error>>{
    let input = env::args()
	.into_iter()
	.skip(1)
	.map(|it| it.parse::<u32>())
	.map(|it| it.expect("You must pass a list of integers by command-line arguments"))
	.collect::<Vec<u32>>();

    let stats = stats::Stats::from(&input);
    println!("{}", stats);
    
    Ok(())
}
