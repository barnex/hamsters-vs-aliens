use super::gen_vec3::*;
use super::ivec_3::*;
use std::ops::Mul;

#[allow(non_camel_case_types)]
pub type vec3 = gvec3<f32>;

pub const fn vec3(x: f32, y: f32, z: f32) -> vec3 {
	vec3::new(x, y, z)
}

impl vec3 {
	pub fn to_ivec(self) -> ivec3 {
		self.map(|v| v as i32)
	}
}

impl Mul<vec3> for f32 {
	type Output = vec3;

	#[inline]
	fn mul(self, rhs: vec3) -> Self::Output {
		rhs.mul(self)
	}
}
