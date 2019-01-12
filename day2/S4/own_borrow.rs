fn main() {
	// Owning
	let mut my_vector = vec![1, 2, 3, 4, 5];
	my_vector[2] = 100;
	
	let ref another_vector = my_vector;
	
	println!("my_vector is {:?}", my_vector);
	println!("another_vector is {:?}", another_vector);
	
	let integer = Box::new(1235);
	println!("integer is (using pointer) {}", *integer);
	let my_integer = integer;
	println!("my_integer: {}", my_integer);
	
	let mut my_int = 12345;
	// This creates a copy because it is an integer
	let mut my_int2 = my_int;
	println!("my_int is {}", my_int);
	println!("my_int2 is {}", my_int2);
	my_int2 = -100;
	println!("my_int2 is {}", my_int2);
	my_int = -12312312;
	println!("my_int is {}", my_int);
	
	// Borrowing
	let mut a_var = 3.14;
	{
		let b_var = &mut a_var;
		// Error: cannot borrow `a_var` as immutable because it is also borrowed as mutable
		// println!("a_var inside is {}", a_var);
		*b_var = 3.14159;
	}
	println!("a_var is now {}", a_var);
}

