use super::internal::*;
use Message::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct SnowCannon {
	pub snowball_radius: f32,
	last_shoot_time: f32,
}

impl SnowCannon {
	const RECHARGE_TIME: f32 = 0.15;

	pub fn new() -> Self {
		Self {
			snowball_radius: 1.5,
			last_shoot_time: 0.0,
		}
	}

	pub fn fire(&mut self, _dt: f32, (pos, view, dir): WeaponOrientation, gs: &GameState, updates: &mut Updates) {
		// cannot fire faster than once per recharge time
		if gs.time - self.last_shoot_time < Self::RECHARGE_TIME {
			return;
		}
		self.last_shoot_time = gs.time;

		if let Some(hit) = Weapon::hit_point(gs, view, dir, true /*bw_offset*/) {
			self.explode(hit, &gs.map, updates);
			updates.push(Self::snowbeam_effect(pos, hit));
		} else {
			updates.push(Self::snowbeam_effect(pos, view + Weapon::SHOOT_DIST * dir));
		}
	}

	fn explode(&self, center: vec3, map: &Map, updates: &mut Vec<Message>) {
		// slight vertical offset makes it easier to build snow walls
		let center = center + vec3(0.0, 1.0, 0.0);
		let icenter = Map::voxel_index(center);

		let r = self.snowball_radius;

		let min = -3;
		let max = -min + 1;

		for dz in min..max {
			for dy in min..max {
				for dx in min..max {
					let ipos = icenter + ivec3(dx, dy, dz);
					let pos = ipos.map(|v| v as f32 + 0.5);
					let dist = (pos - center).len() + rand(-0.1, 0.1);
					if dist < r && map.at(ipos) == Voxel::EMPTY {
						updates.push(UpdateMap { index: ipos, voxel: Voxel::SNOW });
						updates.push(AddEffect(Self::snow_effect(pos)));
					}
				}
			}
		}
	}

	fn snow_effect(pos: vec3) -> Effect {
		Effect::ParticleEffect(Particle {
			pos,
			vel: rand(2.0, 10.0) * rand_vec(),
			acc: -25.0,
			ttl: rand(0.3, 3.0),
			mesh: 0,
			tex: Voxel::SNOW.tex_id() as u8,
		})
	}

	fn snowbeam_effect(start: vec3, stop: vec3) -> Message {
		AddEffect(Effect::TrailEffect(Trail::new(start, stop, WHITE)))
	}

	pub fn draw(&self, ctx: &GLContext, (pitch, int, yaw, ext): (f32, vec3, f32, vec3)) {
		let shader = ctx.shaders().bind_anim_shader();
		shader.set_transform(pitch, int, yaw, ext);
		ctx.textures().bind_voxels();
		shader.set_texture(Voxel::SNOW.tex_id());
		ctx.meshes().weapon.bind_and_draw();
	}
}
