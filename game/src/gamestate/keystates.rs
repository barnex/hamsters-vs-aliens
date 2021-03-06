use super::prelude::*;

/// KeyStates stores which Key is currently pressed.
/// Used for debouncing, removing key repeats.
pub struct KeyStates {
	pressed: [bool; NUM_KEYS],
	released: [bool; NUM_KEYS],
	down: [bool; NUM_KEYS],
	mouse_delta: (f32, f32),
}

impl KeyStates {
	pub fn new() -> Self {
		Self {
			pressed: [false; NUM_KEYS],
			released: [false; NUM_KEYS],
			down: [false; NUM_KEYS],
			mouse_delta: (0.0, 0.0),
		}
	}

	/// Record that `key` was pressed or released.
	pub fn record(&mut self, key: Key, pressed: bool) {
		if pressed {
			self.pressed[key as usize] = true;
			self.down[key as usize] = true;
		} else {
			self.released[key as usize] = true;
		}
	}

	/// Record that the mouse was moved by `(delta_x, delta_y)`.
	pub fn record_mouse(&mut self, delta: (f64, f64)) {
		self.mouse_delta.0 += delta.0 as f32;
		self.mouse_delta.1 += delta.1 as f32;
	}

	/// Must be called at the end of each game tick
	/// to clear pressed/released states.
	pub fn clear(&mut self) {
		for (i, &r) in self.released.iter().enumerate() {
			if r {
				self.down[i] = false;
			}
		}
		self.pressed = [false; NUM_KEYS];
		self.released = [false; NUM_KEYS];
		self.mouse_delta = (0.0, 0.0);
	}

	/// True if `key` was down at least some time during the last tick.
	/// I.e., very short keypresses are debounced and still recorded as having been down.
	pub fn is_down(&self, key: Key) -> bool {
		self.down[key as usize]
	}

	/// True if `key` transitioned (at least once) from up to down during the last tick.
	pub fn was_pressed(&self, key: Key) -> bool {
		self.pressed[key as usize]
	}

	/// True if `key` transitioned (at least once) from down to up during the last tick.
	pub fn was_released(&self, key: Key) -> bool {
		self.released[key as usize]
	}

	pub fn mouse_delta(&self) -> (f32, f32) {
		self.mouse_delta
	}
}
