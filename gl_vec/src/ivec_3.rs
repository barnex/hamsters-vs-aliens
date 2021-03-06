use super::gen_vec3::*;
use super::vec_3::vec3;
use std::ops::Mul;

#[allow(non_camel_case_types)]
pub type ivec3 = gvec3<i32>;

pub const fn ivec3(x: i32, y: i32, z: i32) -> ivec3 {
	ivec3::new(x, y, z)
}

impl ivec3 {
	pub fn to_vec(self) -> vec3 {
		self.map(|v| v as f32)
	}
}

impl Mul<ivec3> for i32 {
	type Output = ivec3;

	#[inline]
	fn mul(self, rhs: ivec3) -> Self::Output {
		rhs.mul(self)
	}
}
