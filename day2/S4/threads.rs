
use std::thread;
use std::time::Duration;

fn main() {
	let mut my_threads = vec![];
	for i in 0..15 {
		let join_handle = thread::spawn( move || {
			thread::sleep(Duration::from_millis(4000));
			println!("Hello from thread {}", i);
		});
		my_threads.push(join_handle);
	}
	
	for join_handle in my_threads {
		let _ = join_handle.join();
	}
	println!("Done!");
}
