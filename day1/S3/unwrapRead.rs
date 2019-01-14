use std::io;

fn main()
{
	// Read the price
	println!("Give me the price");
	let price = io::stdin().read_line().unwrap();
	println!("The price is {}", price);
}
