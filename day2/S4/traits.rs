use std::ops::Add;

#[derive(Debug)]
pub struct PointInSpace {
	x: i32,
	y: i32,
	z: i32,
}

impl Add for PointInSpace {
	type Output = PointInSpace;
	fn add(self, second:PointInSpace) -> Self::Output {
		PointInSpace {
			x: self.x + second.x,
			y: self.y + second.y,
			z: self.z + second.z,
		}
	}
}

fn main() {
	let a1: PointInSpace = PointInSpace{x:1, y:2, z:3};
	println!("a1: {:?}", a1);
	let a2: PointInSpace = PointInSpace{x:-1, y:3, z:-10};

	let k = 1 + 3;
	println!("k: {}", k);
	
	let a3 = a1 + a2;
	println!("a3: {:?}", a3);
}
