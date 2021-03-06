//! The core game logic.

pub mod prelude;

// TODO: make private when crate::* has moved into game state
pub mod internal;

mod boundingbox;
mod effect;
mod gamestate;
mod key;
mod keystates;
mod map;
mod message;
mod model;
mod player;
mod players;
mod weapons;
