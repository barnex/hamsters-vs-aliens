use super::gen_vec3::*;
use std::ops::Mul;

#[allow(non_camel_case_types)]
pub type dvec3 = gvec3<f64>;

pub const fn dvec3(x: f64, y: f64, z: f64) -> dvec3 {
	dvec3::new(x, y, z)
}

impl Mul<dvec3> for f64 {
	type Output = dvec3;

	#[inline]
	fn mul(self, rhs: dvec3) -> Self::Output {
		rhs.mul(self)
	}
}
