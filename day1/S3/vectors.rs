
fn main() {
	// Create a new _empty_ Vector
	let mut v: Vec<i64> = Vec::new();
	println!("v: {:?}", v);

	// Add some values to it!
	v = vec![1, 2, 3];
	
	// This will generate an error message:
	// v1 = vec![1, 2, 3];
	
	println!("v: {:?}", v);
	
	{
		// Create a new _empty_ Vector
		let mut v1: Vec<i64> = Vec::new();
		println!("v1: {:?}", v1);
		
		v1.push(1);
		v1.push(2);
		// Duplicates are allowed
		v1.push(1);
		println!("v1: {:?}", v1);
	}
	
	// Out of scope!
	// println!("v1: {:?}", v1);

	// let will_crash_if_not_there = &v[50];
	// let will_not_crash_if_not_there = v.get(50);
	
	for el in v {
		println!("el: {}", el);
	}
	
	v = vec![1, 2, 3];	
	for x in v.iter() {
	    println!("iter: {}", x);
	}

	let mut v1: Vec<i64> = vec![2, -2, 2];
	for el in &mut v1 {
		*el += 10;
	}
	println!("v1: {:?}", v1);
}
