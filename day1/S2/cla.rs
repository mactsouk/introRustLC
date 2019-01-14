use std::env;

fn main() {
	// println!("{:?}:", env::args());
	for argument in env::args() {
    	println!("{}", argument);
	}
}
