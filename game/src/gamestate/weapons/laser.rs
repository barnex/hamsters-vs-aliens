use super::internal::*;
use Message::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct Laser {
	pub obliterate_radius: f32,
	pub melt_radius: f32,
	last_shoot_time: f32,
}

impl Laser {
	const RECHARGE_TIME: f32 = 0.5;

	pub fn new() -> Self {
		Self {
			obliterate_radius: 2.0,
			melt_radius: 3.3,
			last_shoot_time: 0.0,
		}
	}

	pub fn fire(&mut self, _dt: f32, (pos, view, dir): WeaponOrientation, gs: &GameState, updates: &mut Updates) {
		// cannot fire faster than once per recharge time
		if gs.time - self.last_shoot_time < Self::RECHARGE_TIME {
			return;
		}
		self.last_shoot_time = gs.time;

		if let Some(hit) = Weapon::hit_point(gs, view, dir, false /*bw_offset*/) {
			self.explode(hit, &gs.map, updates);
			updates.push(self.laserbeam_effect(pos, hit));
		} else {
			updates.push(self.laserbeam_effect(pos, view + Weapon::SHOOT_DIST * dir));
		}
	}

	fn explode(&self, center: vec3, map: &Map, updates: &mut Vec<Message>) {
		let icenter = Map::voxel_index(center);

		let r_outer = self.melt_radius;
		let r_inner = self.obliterate_radius;

		let min = -3;
		let max = -min + 1;

		for dz in min..max {
			for dy in min..max {
				for dx in min..max {
					let ipos = icenter + ivec3(dx, dy, dz);
					let pos = ipos.map(|v| v as f32 + 0.5);
					let dist = (pos - center).len() + rand(-0.5, 0.5);
					if dist < r_outer && dist > r_inner && map.at(ipos) != Voxel::EMPTY {
						updates.push(UpdateMap { index: ipos, voxel: Voxel::LAVA });
					}
					if dist < r_inner && map.at(ipos) != Voxel::EMPTY {
						updates.push(UpdateMap { index: ipos, voxel: Voxel::EMPTY });
						updates.push(AddEffect(Self::explosion_effect(pos)));
					}
				}
			}
		}
	}

	fn explosion_effect(pos: vec3) -> Effect {
		Effect::ParticleEffect(Particle {
			pos,
			vel: rand(0.0, 25.0) * (rand_vec() + vec3(0.0, 1.0, 0.0)),
			acc: -25.0,
			ttl: rand(0.3, 3.0),
			mesh: 0,
			tex: Voxel::LAVA.tex_id() as u8,
		})
	}

	fn laserbeam_effect(&self, start: vec3, stop: vec3) -> Message {
		AddEffect(Effect::TrailEffect(Trail::new(start, stop, RED)))
	}

	pub fn draw(&self, ctx: &GLContext, (pitch, int, yaw, ext): (f32, vec3, f32, vec3)) {
		// TODO: Weapon::draw()
		let shader = ctx.shaders().bind_anim_shader();
		shader.set_transform(pitch, int, yaw, ext);
		ctx.textures().bind_voxels();
		shader.set_texture(Voxel::LAVA.tex_id());
		ctx.meshes().weapon.bind_and_draw();
	}
}
