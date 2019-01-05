use std::thread;

fn main() {
	let my_var = -100;
	thread::spawn( move || {
		println!("The value is {:?}", my_var);
	});
	println!("Done!");
}

