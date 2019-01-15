extern crate rayon;
use rayon::prelude::*;
use std::env;

fn fibonacci(n: i32) -> i32 {
	if n == 0 {
		return 0;
	}
	if n <= 1 {
		return 1;
	} 
	else {
		return fibonacci(n - 1) + fibonacci(n - 2);
	}
}

fn main() {
	let args: Vec<_> = env::args().collect();
	if args.len() != 2 {
 		println!("Usage: {} n", args[0]);
        return;
 	}

	let n: i32;
	let m = &args[1];
    match m.parse::<i32>() {
      Ok(n1) => n = n1,
      Err(e) => panic!("{} is NOT a number!", e),
    }

	let v: Vec<i32> = (0..n).collect(); 

	let total: i32 = v
	    // .into_par_iter()
	    .iter()
	    // .map(|x| fibonacci(x))
	    .map(|x| fibonacci(*x))
	    .sum();

	println!("{:?}", total);
}
