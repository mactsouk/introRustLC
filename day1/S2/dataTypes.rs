fn main() {
	
	let x;
	
	// This will not work
	// Using a variable before initialization
	// println!("{}", x);
	
	if 5 > 10 {
		x = 10;
	} else {
		x = 12;
	}
	println!("{}", x);
	
	let xx: i32 = 100;
	println!("xx: {}", xx);
	
	// This is an error - cannot change immutable variable
	// xx = 10;
	
	// This is not an error
	let mut abc: i8 = 8;
	abc = 10;
	println!("abc: {}", abc);
	
	for y in 1..10 {
		let y  = 2 * y;
		println!("{}", y);
	}

	// This will not work:
	// println!("Outside for: {}", y);
}
