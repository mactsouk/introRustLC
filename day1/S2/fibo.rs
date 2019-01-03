// Programmer: Mihalis Tsoukalos
// Calculating Fibonacci numbers in Rust
//
// The numbers are given as command line arguments

use std::env;
use std::str;

fn fibonacci(n: i64) -> i64 {
	if n == 0 {
		return 0;
	}

	if n <= 1 {
		return 1;
	} else {
		return fibonacci(n - 1) + fibonacci(n - 2);
	}
}

fn main() {
	// This is a vector as you can see from the output
	println!("{:?}", env::args());

	for input in env::args().skip(1) {
        let _i = match input.parse::<i64>() {
          Ok(_i) => {
			  println!("Fibonacci number {} is {} ", _i, fibonacci(_i));
          },
          Err(_e) => {
              println!("{}: Not a valid integer!", input)
          }
        };
	}
}
