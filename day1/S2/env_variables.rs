use std::env;

fn main() {
    match env::var("PATH") {
        Ok(value) => println!("Current path: {}", value),
        Err(e) => println!("PATH NOT SET {})", e),
    };
}
