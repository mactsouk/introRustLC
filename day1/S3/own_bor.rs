fn main() {
	// Owning

	// For primitive data types, which are cheaper,
	// Rust performs a full copy but for other types,
	// such as a Vector, it is not allowed to use a variable
	// after you have assigned it to another variable.
	let mut my_vector = vec![1, 2, 3, 4, 5];
	my_vector[2] = 100;
	
	let ref another_vector = my_vector;
	// This is not allowed
	// my_vector[2] = 100;
	
	println!("my_vector is {:?}", my_vector);
	println!("another_vector is {:?}", another_vector);
	
	let integer = Box::new(1235);
	println!("integer is (using pointer) {}", *integer);
	let my_integer = integer;
	println!("my_integer: {}", my_integer);
	
	// This will not work becuase it is
	// immutable variable
	// Error: cannot assign twice to immutable variable `my_integer`
	// my_integer = Box::new(-20000);
	
	// You cannot use this
	// println!("integer is {}", *integer);
	
	// You cannot use this
	// Error: use of moved value: `integer`
	// println!("integer is {}", integer);

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

