// This is just a tuple
#[derive(PartialEq,Debug)]
pub struct METERS (f32);
#[derive(PartialEq,Debug)]
pub struct KILOMETERS (f32);
#[derive(PartialEq,Debug)]
pub struct MILES (f32);

pub trait ToMeters {
	fn to_meters(&self)->METERS;
}

impl ToMeters for KILOMETERS {
	fn to_meters(&self)->METERS {
		METERS(self.0 * 1000.0)
	}
}

impl ToMeters for MILES {
	fn to_meters(&self)->METERS {
		METERS(self.0 * 1609.0)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
    #[test]
    fn it_works() {
		// What is going to happen if you use the following?
		// let kilo = KILOMETERS(10);
        let kilo = KILOMETERS(10.0);
		let meters = kilo.to_meters();
		assert_eq!(meters, METERS(10000.0));
        let miles = MILES(10.0);
		let meters_miles = miles.to_meters();
		assert_eq!(meters_miles, METERS(16090.0));
    }
}
