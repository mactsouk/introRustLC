use std::thread;
use std::time::Duration;

fn main() {
	for i in 0..10 {
		thread::spawn( move || {
			thread::sleep(Duration::from_millis(4000));
			println!("Hello from thread {}!", i);
		});
	}
	println!("Done!");
}

