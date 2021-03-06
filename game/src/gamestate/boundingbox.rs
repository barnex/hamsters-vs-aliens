use super::internal::*;

/// Axis Aligned Box (https://en.wikipedia.org/wiki/Minimum_bounding_box#Axis-aligned_minimum_bounding_box).
#[derive(Clone, Debug, PartialEq)]
pub struct BoundingBox {
	min: vec3,
	max: vec3,
}

impl BoundingBox {
	/// A BoundingBox enclosing points `min` and `max`.
	pub fn new(min: vec3, max: vec3) -> Self {
		debug_assert!(min.x <= max.x);
		debug_assert!(min.y <= max.y);
		debug_assert!(min.z <= max.z);
		Self { min, max }
	}

	pub fn min(&self) -> vec3 {
		self.min
	}
	pub fn max(&self) -> vec3 {
		self.max
	}

	/// The bounding box's center.
	pub fn center(&self) -> vec3 {
		(self.min + self.max) * 0.5
	}

	pub fn size(&self) -> vec3 {
		self.max - self.min
	}

	pub fn from<'a, T: Iterator<Item = &'a vec3>>(mut positions: T) -> Self {
		let first = positions.next().expect("BoundingBox::from: illegal argument: zero positions");
		let mut bb = Self::new(*first, *first);
		for pos in positions {
			bb.add(*pos)
		}
		bb
	}

	fn add(&mut self, p: vec3) {
		self.min.x = f32::min(self.min.x, p.x);
		self.min.y = f32::min(self.min.y, p.y);
		self.min.z = f32::min(self.min.z, p.z);

		self.max.x = f32::max(self.max.x, p.x);
		self.max.y = f32::max(self.max.y, p.y);
		self.max.z = f32::max(self.max.z, p.z);
	}
}
