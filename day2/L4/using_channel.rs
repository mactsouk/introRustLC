use std::env;
use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};

fn fibonacci(n: i32) -> i32 {
	if n == 0 {
		return 0;
	}
	if n <= 1 {
		return 1;
	} 
	else {
		return fibonacci(n - 1) + fibonacci(n - 2);
	}
}

fn main() {
	let args: Vec<_> = env::args().collect();
	if args.len() != 2 {
 		println!("Usage: {} n", args[0]);
        return;
 	}

	let n: i32;
	let m = &args[1];
    match m.parse::<i32>() {
      Ok(n1) => n = n1,
      Err(e) => panic!("{} is NOT a number!", e),
    }
	println!("Going to create {} threads!", n);

	let (tx, rx): (Sender<i32>, Receiver<i32>) = channel();
	for i in 0..n {
		let thread_tx = tx.clone();
		
		thread::spawn( move || {
			let n = fibonacci(i);
			thread_tx.send(n).unwrap();
		});
	}

	let mut f = Vec::with_capacity(n as usize);
    for _ in 0..n {
        f.push(rx.recv());
    }
	
	let mut sum = 0;
	for k in f {
		sum = sum + k.unwrap();
		print!("{} ", k.unwrap());
	}
	println!("\nSum = {}.", sum);
}

