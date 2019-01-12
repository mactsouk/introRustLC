#[derive(PartialEq,Debug)]
pub struct METERS (f32);
#[derive(PartialEq,Debug)]
pub struct KILOMETERS (f32);
#[derive(PartialEq,Debug)]
pub struct MILES (f32);

pub trait ToMeters<M> {
	fn to_meters(&self, _:M) -> f32;
}

pub trait FromMeters<M> {
	fn from_meters(&self, _:f32) -> M;
}

pub struct Convert {
	km:f32,
	miles:f32,
}

impl ToMeters<KILOMETERS> for Convert {
	fn to_meters(&self, d:KILOMETERS) -> f32 {
		(d.0 as f32) * self.km
	}
}

impl FromMeters<KILOMETERS> for Convert {
	fn from_meters(&self, d:f32) -> KILOMETERS {
		KILOMETERS( (d / self.km) as f32 )
	}
}

impl ToMeters<MILES> for Convert {
	fn to_meters(&self, d:MILES) -> f32 {
		(d.0 as f32) * self.miles
	}
}

impl FromMeters<MILES> for Convert {
	fn from_meters(&self, d:f32) -> MILES {
		MILES( (d / self.miles) as f32 )
	}
}

#[cfg(test)]
mod tests {
	use super::*;
    #[test]
    fn it_works() {
		let convert = Convert{km:1000.0, miles:1634.0};

        let k:KILOMETERS = convert.from_meters(1000.0);		
		assert_eq!(k, KILOMETERS(1.0));
		
		let m = MILES(1.0);
        let k = convert.to_meters(m);
		assert_eq!(k, 1634.0);
		// This will fail:
		// assert_eq!(k, 1634.0);
    }
}
