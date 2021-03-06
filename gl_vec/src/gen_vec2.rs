use super::float::*;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::*;

#[derive(Clone, Copy, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct gvec2<T> {
	pub x: T,
	pub y: T,
}

impl<T> gvec2<T> {
	#[inline]
	pub const fn new(x: T, y: T) -> Self {
		Self { x, y }
	}
}

impl<T> PartialEq for gvec2<T>
where
	T: PartialEq + Copy,
{
	#[inline]
	fn eq(&self, rhs: &Self) -> bool {
		self.x == rhs.x && self.y == rhs.y
	}
}

impl<T> Add for gvec2<T>
where
	T: Add<T, Output = T> + Copy,
{
	type Output = Self;

	#[inline]
	fn add(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		}
	}
}

impl<T> AddAssign for gvec2<T>
where
	T: AddAssign + Copy,
{
	#[inline]
	fn add_assign(&mut self, rhs: Self) {
		self.x += rhs.x;
		self.y += rhs.y;
	}
}

impl<T> Div<T> for gvec2<T>
where
	T: Div<T, Output = T> + Copy,
{
	type Output = Self;

	#[inline]
	fn div(self, rhs: T) -> Self::Output {
		Self { x: self.x / rhs, y: self.y / rhs }
	}
}

impl<T> Mul<T> for gvec2<T>
where
	T: Mul<T, Output = T> + Copy,
{
	type Output = Self;

	#[inline]
	fn mul(self, rhs: T) -> Self::Output {
		Self { x: self.x * rhs, y: self.y * rhs }
	}
}

impl<T> MulAssign<T> for gvec2<T>
where
	T: MulAssign + Copy,
{
	#[inline]
	fn mul_assign(&mut self, rhs: T) {
		self.x *= rhs;
		self.y *= rhs;
	}
}

impl<T> Neg for gvec2<T>
where
	T: Neg<Output = T> + Copy,
{
	type Output = Self;

	#[inline]
	fn neg(self) -> Self::Output {
		Self { x: -self.x, y: -self.y }
	}
}

impl<T> Sub for gvec2<T>
where
	T: Sub<T, Output = T> + Copy,
{
	type Output = Self;

	#[inline]
	fn sub(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
		}
	}
}

impl<T> SubAssign for gvec2<T>
where
	T: SubAssign + Copy,
{
	#[inline]
	fn sub_assign(&mut self, rhs: Self) {
		self.x -= rhs.x;
		self.y -= rhs.y;
	}
}

impl<T> Display for gvec2<T>
where
	T: Copy + Display,
{
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "({}, {})", self.x, self.y)
	}
}

impl<T> Debug for gvec2<T>
where
	T: Copy + Debug,
{
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "({:?}, {:?})", self.x, self.y)
	}
}

impl<T> gvec2<T>
where
	T: Add<T, Output = T> + Mul<T, Output = T> + Sub<T, Output = T> + Copy,
{
	/// Dot (inner) product.
	#[inline]
	pub fn dot(self, rhs: gvec2<T>) -> T {
		self.x * rhs.x + self.y * rhs.y
	}

	/// Length squared (norm squared).
	#[inline]
	pub fn len2(self) -> T {
		self.dot(self)
	}
}

impl<T> gvec2<T>
where
	T: Add<T, Output = T> + Mul<T, Output = T> + Sub<T, Output = T> + Div<T, Output = T> + Copy + Float,
{
	/// Length (norm).
	#[inline]
	pub fn len(self) -> T {
		self.len2().sqrt()
	}

	/// Returns a vector with the same direction but unit length.
	#[inline]
	#[must_use]
	pub fn normalized(self) -> Self {
		self * (T::ONE / self.len())
	}

	/// Re-scale the vector to unit length.
	#[inline]
	pub fn normalize(&mut self) {
		*self = self.normalized()
	}

	/// The zero vector.
	pub const ZERO: Self = Self { x: T::ZERO, y: T::ZERO };

	/// All ones.
	pub const ONES: Self = Self { x: T::ONE, y: T::ONE };

	/// Unit vector along X.
	pub const EX: Self = Self { x: T::ONE, y: T::ZERO };

	/// Unit vector along Y.
	pub const EY: Self = Self { x: T::ZERO, y: T::ONE };
}

impl<T> Into<(T, T)> for gvec2<T> {
	fn into(self) -> (T, T) {
		(self.x, self.y)
	}
}

impl<T> gvec2<T>
where
	T: Copy,
{
	#[must_use]
	#[inline]
	pub fn map<F, U>(&self, f: F) -> gvec2<U>
	where
		F: Fn(T) -> U,
	{
		gvec2 { x: f(self.x), y: f(self.y) }
	}
}
