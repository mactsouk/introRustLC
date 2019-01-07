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
    let n: i32 = 10;
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
