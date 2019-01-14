use std::env;
use std::str;

fn main() {
    let mut acc: i64 = 0;
    for n in env::args().skip(1) {
        match str::parse::<i64>(&n) {
            Ok(i) => {
                acc = acc + i;
            },
            Err(e) => {
                println!("Invalid argument {}: {}", n, e);
            }
        }
    }
    println!("Total: {}", &acc);
}

