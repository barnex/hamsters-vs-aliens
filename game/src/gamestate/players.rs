use super::internal::*;
use std::collections::hash_map::{Iter, IterMut};

/// Set of players, indexed by their ID.
#[derive(Serialize, Deserialize, Clone)]
pub struct Players(HashMap<ID, Player>);

/// Player ID, named for clarity.
pub type ID = usize;

impl Players {
	pub fn new() -> Self {
		Self(HashMap::default())
	}

	pub fn set(&mut self, player_id: ID, player: Player) {
		self.0.insert(player_id, player);
	}

	pub fn get(&self, player_id: ID) -> &Player {
		&self.0[&player_id]
	}

	pub fn get_mut(&mut self, player_id: ID) -> &mut Player {
		self.0.get_mut(&player_id).expect("BUG: player not found")
	}

	pub fn iter_mut(&mut self) -> IterMut<ID, Player> {
		self.0.iter_mut()
	}

	pub fn iter(&self) -> Iter<ID, Player> {
		self.0.iter()
	}

	pub fn remove(&mut self, player_id: ID) {
		self.0.remove(&player_id).expect("BUG: player not found");
	}

	pub fn len(&self) -> usize {
		self.0.len()
	}

	pub fn draw(&self, player_id: usize, ctx: &GLContext) {
		ctx.textures().bind_skins();
		for (&i, player) in self.0.iter() {
			if i == player_id {
				player.draw_first_person(ctx)
			} else {
				player.draw_third_person(ctx)
			}
		}
	}
}
