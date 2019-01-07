fn my_match( num: i32) {
	match num {
		// This will not work:
		//	< 0  => println!("Less than zero"),
		// This is called a match guard
		num if num < 0  => println!("Less than zero"),
  		0     => println!("zero"),
  		1 | 2 | 3 | 4  => println!("one or two or three or four"),
  		5..=20 => println!("five to twenty"),
  		_     => println!("something BIG")
	}
}

fn main() {
	my_match(-100);
	my_match(0);
	my_match(4);
	my_match(14);
	my_match(22);
}
