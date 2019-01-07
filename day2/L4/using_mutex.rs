use std::sync::{Arc, Mutex};
use std::thread;

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
    let sum = Arc::new(Mutex::new(0));
    let mut threads_vec = vec![];
    
    for i in 0..45 {
        let sum = sum.clone();
        let t = thread::spawn(move || {
            let mut total = sum.lock().unwrap();
            *total += fibonacci(i);
        });
        threads_vec.push(t);
    }
    
    for thread in threads_vec {
        thread.join().unwrap();
    }
    println!("Summary: {}", *sum.lock().unwrap());
}
