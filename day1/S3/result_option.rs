
fn main() {
	
	let mut input = "123";
	
    let _i = match input.parse::<i32>() {
      Ok(_i) => {
          println!("i: {}", _i);
      },
      Err(_e) => {
          println!("{}: Not a valid integer!", input);
      }
    };

	input = "abc";
	
    let _i = match input.parse::<i32>() {
      Ok(_i) => {
          println!("i: {}", _i);
      },
      Err(_e) => {
          println!("{}: Not a valid integer!", input);
      }
    };

	
}