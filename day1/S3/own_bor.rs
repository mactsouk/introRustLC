fn main() {
	// Owning

	// For primitive data types, which are cheaper,
	// Rust performs a full copy but for other types,
	// such as a Vector, it is not allowed to use a variable
	// after you have assigned it to another variable.
	let my_vector = vec![1, 2, 3, 4, 5];
	let ref another_vector = my_vector;
	
	println!("my_vector is {:?}", my_vector);
	
	let integer = Box::new(12345);
	let my_integer = integer;
	
	// You cannot use this!
	// println!("integer is {}", *integer);
	
	// Borrowing
	let mut a_var = 3.14;
	{
		let b_var = &mut a_var;
		*b_var = 3.14159;
	}
	println!("a_var is now {}", a_var);
}

