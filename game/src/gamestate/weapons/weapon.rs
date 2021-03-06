use super::internal::*;

#[derive(Serialize, Deserialize, Clone)]
pub enum Weapon {
	Laser(Laser),
	SnowCannon(SnowCannon),
	EditGun(EditGun),
}

/// Nozzle, Camera, Look direction.
pub type WeaponOrientation = (vec3, vec3, vec3);

impl Weapon {
	pub fn laser() -> Weapon {
		Weapon::Laser(Laser::new())
	}

	pub fn snow_cannon() -> Weapon {
		Weapon::SnowCannon(SnowCannon::new())
	}

	pub fn edit_gun() -> Weapon {
		Weapon::EditGun(EditGun::new())
	}

	pub fn fire(&mut self, dt: f32, alt: bool, orientation: WeaponOrientation, gs: &GameState, updates: &mut Updates) {
		match self {
			Weapon::Laser(w) => w.fire(dt, orientation, gs, updates),
			Weapon::SnowCannon(w) => w.fire(dt, orientation, gs, updates),
			Weapon::EditGun(w) => w.fire(dt, alt, orientation, gs, updates),
		}
	}

	pub fn draw(&self, ctx: &GLContext, orientation: (f32, vec3, f32, vec3)) {
		match self {
			Weapon::Laser(w) => w.draw(ctx, orientation),
			Weapon::SnowCannon(w) => w.draw(ctx, orientation),
			Weapon::EditGun(w) => w.draw(ctx, orientation),
		}
	}

	pub const SHOOT_DIST: f32 = 300.0;

	pub fn hit_point(gs: &GameState, start: vec3, dir: vec3, bw_offset: bool) -> Option<vec3> {
		gs.map().intersect(start, dir, Self::SHOOT_DIST, bw_offset)
		// let map = &gs.map;
		// // TODO: hit other players;
		// let delta = 0.1;
		// let mut t = 0.0;
		// for _i in 0..1000 {
		// 	// TODO: observe SHOOT_DIST
		// 	t += delta;
		// 	let probe = start + t * dir;
		// 	let probe_idx = Map::voxel_index(probe);
		// 	if map.at(probe_idx) != Voxel::EMPTY {
		// 		if bw_offset {
		// 			return Some(start + (t - delta) * dir);
		// 		} else {
		// 			return Some(probe);
		// 		}
		// 	}
		// }
		// None
	}
}
