use std::thread;
use std::time::Duration;

fn main() {
	for i in 0..5 {
		thread::spawn( move || {
			thread::sleep(Duration::from_millis(4000));
			println!("Hello from thread {}!", i);
		});
	}
	thread::sleep(Duration::from_millis(5000));
	println!("Done!");
}

