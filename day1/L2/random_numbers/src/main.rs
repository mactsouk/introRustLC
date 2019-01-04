extern crate rand;

use rand::Rng;
use std::env;

fn main(){
    let args: Vec<_> = env::args().collect();
	
	let mut min = 0;
	let mut max = 0;
	let mut n = 0;

    if args.len() >= 4 {
		let i1 = ::std::env::args().nth(1).unwrap();
		let i2 = ::std::env::args().nth(2).unwrap();
		let i3 = ::std::env::args().nth(3).unwrap();
	
		let _i = match i1.parse::<i32>() {
			Ok(_i) => {
				min = _i;
			},
			Err(_e) => {
            	print!("{}: Not a valid integer!", i1)
        	}
		};

		let _i = match i2.parse::<i32>() {
			Ok(_i) => {
				max = _i;
			},
			Err(_e) => {
            	print!("{}: Not a valid integer!", i2)
        	}
		};
		
		let _i = match i3.parse::<i32>() {
			Ok(_i) => {
				n = _i;
			},
			Err(_e) => {
            	print!("{}: Not a valid integer!", i3)
        	}
		};
		
		for _ in 0..n {
		    print!("{} ", rand::thread_rng().gen_range(min, max+1));
		}		
	} else {
       	print!("Not enough command line arguments")
	}
	println!();
}
