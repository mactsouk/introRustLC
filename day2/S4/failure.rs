use std::thread;

fn main() {
	let my_var = -100;
	thread::spawn( || {
		println!("The value is {:?}", my_var);
	});
	println!("Done!");
}

