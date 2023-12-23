const ERROR_MESSAGE: &'static str = "You must provide a number by command line arguments";

fn main() {
    let n = std::env::args().collect::<Vec<String>>().get(1)
	.expect(ERROR_MESSAGE)
	.parse::<u64>()
	.expect(ERROR_MESSAGE)
	;
    
    let result = recursive_fib(n);
    
    println!("{result}");
}

fn recursive_fib(n: u64) -> u64 {
    if n <= 1 {
	n
    } else {
	recursive_fib(n-1) + recursive_fib(n-2)
    }
}
