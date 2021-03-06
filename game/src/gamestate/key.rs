/// Key codes with meaning internal to the game.
/// Mapped from physical keys (see `keymap`).
#[derive(Copy, Clone, Debug)]
pub enum Key {
	Left = 0,
	Right = 1,
	Forward = 2,
	Backward = 3,
	Jump = 4,
	Crouch = 5,
	Sprint = 6,
	Mouse1 = 7,
	Mouse3 = 8,
	// TODO: fill gap
	Key1 = 11,
	Key2 = 12,
	Key3 = 13,
	Key4 = 14,
	Key5 = 15,
	Key6 = 16,
	Key7 = 17,
	Key8 = 18,
	Key9 = 19,
	Key0 = 20,
}
use Key::*;

impl Key {
	pub const NUMERIC_KEYS: [Key; 10] = [Key1, Key2, Key3, Key4, Key5, Key6, Key7, Key8, Key9, Key0];
}

pub const NUM_KEYS: usize = 21;
