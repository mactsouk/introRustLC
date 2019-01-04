
fn bigger(a: u32, b:i64) -> i64 {	
	let mut result = a as i64;
	if result < b {
		result = b;
	}
	
	// The following are the same:
	// return result;
	result
}

fn main() {
	let a = bigger(10, -200);
	let b = bigger(100, 2);
	
	println!("a: {}", a);
	println!("b: {}", b);
}