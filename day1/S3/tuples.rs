fn return_tuple() -> (i32, bool) {
	return (100, true);
}

fn main() {
	let t1: (i32, i64, bool) = (10, -100, true);
	println!("t1: {:?}", t1);

	println!("return_tuple(): {:?}", return_tuple());
}
