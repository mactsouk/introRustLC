// This was suggested by the Rust compiler
// for being able to print the structure.
#[derive(Debug)]

// Or pub to make it public, which is not the case here
struct Limits {
	name: String,
	min: i32,
	max: i32,
}

fn main() {

	let l = Limits {
		name:"Grades".to_string(),
		min: 0,
		max: 20,
	};

	println!("{:?}", l);
	// What will happen if you use the following?
	// println!("{?}", l);
}
