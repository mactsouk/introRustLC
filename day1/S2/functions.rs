fn bigger(a: u32, b:i64) -> i64 {	
	let mut result = a as i64;
	if result < b {
		result = b;
	}
	
	// The following are the same:
	// return result;
	result
}

fn check_type(a: i32) {
	println!("a: {}", a);
}

fn main() {
	let a = bigger(10, -200);
	let b = bigger(100, 2);
	
	println!("a: {}", a);
	println!("b: {}", b);
	
	let x: i64 = 10;
	// This is OK
	check_type(x as i32);
	// This is not OK!
	// check_type(x);
	
	let xx = 10;
	println!("Outside xx: {}", xx);
	
	let yy = {
		let xx = 15;
		println!("xx: {}", xx);
		xx + 10
	};
	println!("yy: {}", yy);

	// This is different!
	let yyy = {
		let xx = 15;
		println!("xx: {}", xx);
		// This where the problem is
		xx + 10;
	};

	// This is different!
	println!("yyy: {:?}", yyy);
	
	println!("Outside xx: {}", xx);
}
