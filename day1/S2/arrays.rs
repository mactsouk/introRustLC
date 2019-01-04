
fn main() {
    // Fixed-size array
    let a1: [i64; 4] = [-1, 1, 2, 3];

    // Initialiazing an array
    let a2: [i32; 1500] = [0; 1500];

    // Indexing starts at 0
    println!("First element: {}", a1[0]);
    println!("Second element: {}", a1[1]);

	// len() for array length
    println!("Array size: {}", a2.len());

    // Create a slice from an array
    let s1 = &a2[3 .. 10];
	println!("{:?}", s1);

	// len() for slice length
    println!("Slice size: {}", s1.len());

    // Error:
    // println!("{}", a1[5]);
}