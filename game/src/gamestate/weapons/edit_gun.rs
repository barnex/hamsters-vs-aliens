use super::internal::*;
use Message::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct EditGun {
	selected_block: Voxel,
}

impl EditGun {
	pub fn new() -> Self {
		Self { selected_block: Voxel::from(1) }
	}

	pub fn fire(&mut self, _dt: f32, alt: bool, orientation: WeaponOrientation, gs: &GameState, updates: &mut Updates) {
		//self.control_selected_block(keys); // TODO
		if !alt {
			self.add_block(orientation, gs, updates)
		} else {
			self.remove_block(orientation, gs, updates)
		}
	}

	pub fn draw(&self, _ctx: &GLContext, _orientation: (f32, vec3, f32, vec3)) {
		// not visible
	}

	fn add_block(&mut self, (_pos, view, dir): WeaponOrientation, gs: &GameState, updates: &mut Updates) {
		if let Some(hit) = Weapon::hit_point(gs, view, dir, true /*bw_offset*/) {
			updates.push(UpdateMap {
				index: Map::voxel_index(hit),
				voxel: self.selected_block,
			});
		}
	}

	fn remove_block(&mut self, (_pos, view, dir): WeaponOrientation, gs: &GameState, updates: &mut Updates) {
		if let Some(hit) = Weapon::hit_point(gs, view, dir, false /*bw_offset*/) {
			updates.push(UpdateMap {
				index: Map::voxel_index(hit),
				voxel: Voxel::EMPTY,
			});
		}
		// TODO: debris effect
	}

	fn control_selected_block(&mut self, keys: &KeyStates) {
		for (i, &k) in Key::NUMERIC_KEYS.iter().enumerate() {
			if keys.was_pressed(k) {
				self.selected_block = Voxel::from((i + 1) as u8);
			}
		}
	}
}
