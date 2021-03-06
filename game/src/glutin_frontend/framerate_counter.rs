//use super::internal::*;
use std::time::Instant;

/// Measures and displays frames per second.
pub struct FramerateCounter {
	print: bool,
	last_tick: Instant,
}

impl FramerateCounter {
	pub fn new(print: bool) -> Self {
		FramerateCounter { print, last_tick: Instant::now() }
	}

	pub fn tick(&mut self) {
		let new_tick = Instant::now();
		if self.print {
			let frame_time = new_tick - self.last_tick;
			println!("{:.1} ms = {:.1} FPS", frame_time.as_secs_f64() * 1000.0, 1.0 / frame_time.as_secs_f64());
		}
		self.last_tick = new_tick;
	}
}
