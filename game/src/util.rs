use super::prelude::*;
use rand::Rng;

// clamp a value to lie between min and max (inclusive).
#[inline]
pub fn clamp<T>(v: T, min: T, max: T) -> T
where
	T: Copy + PartialOrd,
{
	debug_assert!(max >= min);
	if v < min {
		return min;
	}
	if v > max {
		return max;
	}
	v
}

pub fn zeros<T: Default>(n: usize) -> Vec<T> {
	let mut dst = Vec::with_capacity(n);
	for _i in 0..n {
		dst.push(T::default());
	}
	dst
}

pub fn wrap_angle(angle: f32) -> f32 {
	if angle > PI {
		return angle - 2.0 * PI;
	}
	if angle < -PI {
		return angle + 2.0 * PI;
	}
	angle
}

pub fn rand(min: f32, max: f32) -> f32 {
	rand::thread_rng().gen_range(min..max)
}

pub const INF: f32 = f32::INFINITY;
