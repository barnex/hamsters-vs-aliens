use crate::prelude::Key;
use glutin::event::VirtualKeyCode;

/// Map physical keys (e.g. 'A') to game-specific keys (e.g.: "Jump").
pub fn keymap(code: VirtualKeyCode) -> Option<Key> {
	use glutin::event::VirtualKeyCode::*;
	match code {
		S | Left => Some(Key::Left),
		F | Right => Some(Key::Right),
		E | Up => Some(Key::Forward),
		D | Down => Some(Key::Backward),
		Space => Some(Key::Jump),
		Z | B => Some(Key::Crouch),
		A => Some(Key::Sprint),
		Key1 => Some(Key::Key1),
		Key2 => Some(Key::Key2),
		Key3 => Some(Key::Key3),
		Key4 => Some(Key::Key4),
		Key5 => Some(Key::Key5),
		Key6 => Some(Key::Key6),
		Key7 => Some(Key::Key7),
		Key8 => Some(Key::Key8),
		Key9 => Some(Key::Key9),
		Key0 => Some(Key::Key0),
		_ => None,
	}
}
