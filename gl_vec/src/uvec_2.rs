use super::gen_vec2::*;
use super::ivec_2::*;
use std::ops::Mul;

#[allow(non_camel_case_types)]
pub type uvec2 = gvec2<u32>;

pub const fn uvec2(x: u32, y: u32) -> uvec2 {
	uvec2::new(x, y)
}

impl Mul<uvec2> for u32 {
	type Output = uvec2;

	#[inline]
	fn mul(self, rhs: uvec2) -> Self::Output {
		rhs.mul(self)
	}
}

impl uvec2 {
	#[inline]
	pub fn as_ivec(self) -> ivec2 {
		ivec2(self.x as i32, self.y as i32)
	}
}
