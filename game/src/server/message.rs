use super::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::io::prelude::*;

/// Messages broadcast by the server to clients.
#[derive(Serialize, Deserialize, Clone)]
pub enum Message {
	/// Initial client request
	Join {
		skin: usize,
	},

	/// Serer response to `Join`,
	Accepted {
		player_id: ID,
		players: Players,
		map_data: Vec<u8>,
	},

	/// A small update to the map (e.g. Voxel added/removed).
	/// Applied incrementally on top of the initial map data.
	///
	/// Sent by client and server.
	/// TODO: UpdateMap(MapDelta), Map.apply(MapDelta).
	UpdateMap {
		index: ivec3,
		voxel: Voxel,
	},

	/// New position, velocity, ... for a player.
	/// Replaces previous player data.
	///
	/// Sent by client and server.
	UpdatePlayer {
		player_id: ID,
		player: Player,
	},

	/// Broadcast by the server when a player has disconnected.
	/// Clients should stop rendering this player.
	DropPlayer {
		player_id: ID,
	},

	AddEffect(Effect),
}

impl Message {
	pub fn serialize<W: Write>(&self, w: &mut W) -> Result<()> {
		Ok(bincode::serialize_into(w, &self)?)
	}

	pub fn deserialize<R: Read>(r: &mut R) -> Result<Self> {
		Ok(bincode::deserialize_from(r)?)
	}
}

impl fmt::Display for &Message {
	fn fmt(&self, w: &mut fmt::Formatter) -> std::result::Result<(), fmt::Error> {
		use Message::*;
		let s = match self {
			Join { .. } => "Join",
			Accepted { .. } => "Accepted",
			UpdateMap { .. } => "UpdateMap",
			UpdatePlayer { .. } => "UpdatePlayer",
			DropPlayer { .. } => "DropPlayer",
			AddEffect(_) => "AddEffect",
		};

		write!(w, "{}", s)
	}
}
