use std::env;

fn main() {
    match env::var("PATH") {
        Ok(value) => println!("Current path: {}", value),
        Err(e) => println!("PATH NOT SET {})", e),
    };

	let my_env = "MIHALIS";
	env::set_var(my_env, "My Value");
    match env::var("MIHALIS") {
        Ok(value) => println!("Current path: {}", value),
        Err(e) => println!("MIHALIS NOT SET {})", e),
    };
}
