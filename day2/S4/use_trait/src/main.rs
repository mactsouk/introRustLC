extern crate rand;

use rand::Rng;

#[derive(Debug,Clone,Copy)]
pub struct PointInSpace {
	x: i32,
	y: i32,
	z: i32,
}

impl PointInSpace {
 	fn random() -> Self {
		let mut get_random = rand::thread_rng();
		PointInSpace {
			x: get_random.gen(),
			y: get_random.gen(),
			z: get_random.gen(),
		}
	}
}

fn main() {
	let a1 = PointInSpace::random();
	println!("a1: {:?}", a1);
	let a2 = PointInSpace::random();
	println!("a2: {:?}", a2);
}
