use super::internal::*;

/// The game's full state and core logic.
/// Independent of whether it runs locally, on a server, or in a client.
#[derive(Serialize, Deserialize)]
pub struct GameState {
	// The world players move in and interact with.
	// Can be loaded from / saved to file.
	pub map: Map,

	// List of currently connected players
	// (locally or through a server).
	pub players: Players,

	// Ephemeral visual effects.
	pub effects: Effects,

	// Local time in seconds. Not synced between server and clients.
	pub time: f32,
}

impl GameState {
	pub fn new(map: Map, players: Players) -> Self {
		Self {
			map,
			players,
			effects: Effects::new(),
			time: 0.0,
		}
	}

	// TODO: rm
	pub fn players(&self) -> &Players {
		&self.players
	}

	// TODO: rm
	pub fn map(&self) -> &Map {
		&self.map
	}

	pub fn time(&self) -> f32 {
		self.time
	}

	// TODO: remove
	pub fn update_player(&mut self, player_id: ID, player: Player) {
		self.players.set(player_id, player);
	}

	// TODO: remove
	pub fn update_map(&mut self, index: ivec3, voxel: Voxel) {
		self.map.set(index, voxel)
	}

	// TODO: remove
	pub fn drop_player(&mut self, player_id: ID) {
		self.players.remove(player_id);
	}

	/// Draw the game state from `player_id`'s viewpoint.
	pub fn draw(&self, player_id: ID, ctx: &GLContext) {
		let player = &self.players.get(player_id);
		ctx.set_matrix(player.camera());
		self.map.draw(ctx, player.bottom_pos());
		self.effects.draw(ctx);
		self.players.draw(player_id, ctx);
		ctx.draw_crosshair(); // draws over the rest, no depth test
	}
}
