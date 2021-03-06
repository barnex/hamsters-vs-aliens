use super::internal::*;

pub const GRAVITY: f32 = 15.0;

#[derive(Serialize, Deserialize, Clone)]
pub struct Model {
	pub hsize: f32,
	pub vsize: f32,
	pub pos: vec3, // center bottom
	pub vel: vec3,
	pub yaw: f32, // angle with -Z, CCW.
	pub pitch: f32,
	pub feet_phase: f32, // left foot's walking animation phase angle

	pub head_mesh: usize,
	pub foot_mesh: usize,
	pub skin_tex: usize,
}

const STAIRCLIMB_SPEED: f32 = 1.5 * Player::SPRINT_SPEED;

impl Model {
	pub fn new(hsize: f32, vsize: f32, head_mesh: usize, foot_mesh: usize, skin_tex: usize) -> Self {
		Model {
			hsize,
			vsize,
			pos: vec3::ZERO,
			vel: vec3::ZERO,
			yaw: 0.0,
			pitch: 0.0,
			feet_phase: 0.0,
			head_mesh,
			foot_mesh,
			skin_tex,
			// walking: false,
		}
	}

	pub fn on_ground(&self, map: &Map) -> bool {
		!self.pos_ok(map, self.pos - vec3(0.0, 0.05, 0.0))
	}

	// _________________________________________________________ mutators

	pub fn try_jump(&mut self, map: &Map, jump_speed: f32) {
		if self.on_ground(map) {
			self.vel.y = jump_speed
		}
	}

	pub fn try_walk(&mut self, dt: f32, map: &Map, walk_speed: vec3) {
		if self.on_ground(map) {
			self.vel.x = walk_speed.x;
			self.vel.z = walk_speed.z;
		} else {
			let airctl = 1.0; // blocks / sec2. compare to G
			self.vel += airctl * walk_speed * dt;
			let damp = 0.1;
			self.vel *= 1.0 - damp * dt;
		}
	}

	// ___________________________________________________________ tick

	pub fn tick(&mut self, dt: f32, map: &Map) {
		self.tick_gravity(dt, map);
		self.tick_rescue(dt, map);
		self.tick_respawn(dt, map);
		self.tick_move(dt, map);
		self.tick_anim(dt, map);
		//self.tick_damping(dt, map);
	}

	pub fn extrapolate(&mut self, dt: f32) {
		self.pos += dt * self.vel;
		// TODO: feet pos
	}

	// apply gravitational acceleration
	fn tick_gravity(&mut self, dt: f32, _map: &Map) {
		self.vel.y -= GRAVITY * dt;
		let damp = 0.05;
		self.vel *= 1.0 - damp * dt;
	}

	// rescue player if somehow stuck inside a block: move them up.
	fn tick_rescue(&mut self, dt: f32, map: &Map) {
		if !self.pos_ok(map, self.pos) {
			self.pos.y += STAIRCLIMB_SPEED * dt;
		}
	}

	// rescue player if somehow fallen off the map: reset position.
	// TODO: move to server logic!
	fn tick_respawn(&mut self, _dt: f32, map: &Map) {
		if self.pos.y < -30.0 {
			let (nx, ny, nz) = map.size().map(|v| v as f32).into();
			let pos = vec3(rand(0.0, nx), ny, rand(0.0, nz));
			self.pos = pos;
		}
	}

	// advance animation
	fn tick_anim(&mut self, dt: f32, _map: &Map) {
		if self.vel != vec3::ZERO {
			self.feet_phase += 8.0 * dt;
			if self.feet_phase > 2.0 * PI {
				self.feet_phase -= 2.0 * PI;
			}
		}
	}

	// Try to move along current velocity for time step dt.
	// Zero velocity component if bumping into wall.
	// Do not move through walls.
	fn tick_move(&mut self, dt: f32, map: &Map) {
		let mut xbump = false;
		let mut zbump = false;
		let delta = self.vel * dt;
		let sub_delta = delta / 16.0;

		for _i in 0..16 {
			let dx = vec3(sub_delta.x, 0.0, 0.0);
			let dy = vec3(0.0, sub_delta.y, 0.0);
			let dz = vec3(0.0, 0.0, sub_delta.z);

			if self.pos_ok(map, self.pos + dx) {
				self.pos += dx;
			} else {
				xbump = true;
			}

			if self.pos_ok(map, self.pos + dy) {
				self.pos += dy;
			} else {
				self.vel.y = 0.0;
			}

			if self.pos_ok(map, self.pos + dz) {
				self.pos += dz;
			} else {
				zbump = true;
			}
		}

		// starirclimb
		let on_ground = !self.pos_ok(map, self.pos - vec3(0.0, 0.05, 0.0));
		if (xbump || zbump) && self.vel.y >= 0.0 && (on_ground) {
			let probe_pos = self.pos + vec3(delta.x, 1.1, delta.z); // what if we kept moving horizontally and took one step up?
			if self.pos_ok(map, probe_pos) {
				self.pos += vec3(delta.x, 0.0, delta.z);
			} else {
				if xbump {
					self.vel.x = 0.0;
				}
				if zbump {
					self.vel.z = 0.0;
				}
			}
		}
	}

	// is this player position allowed in the map?
	// I.e. not bumping into blocks.
	pub fn pos_ok(&self, map: &Map, pos: vec3) -> bool {
		!map.bumps(&self.bounds_for(pos))
	}

	// bounding box for a player at position `pos`.
	fn bounds_for(&self, pos: vec3) -> BoundingBox {
		let min = pos - vec3(self.hsize / 2.0, 0.0, self.hsize / 2.0);
		let max = pos + vec3(self.hsize / 2.0, self.vsize, self.hsize / 2.0);
		BoundingBox::new(min, max)
	}

	/// Fully draw this model, as seen from a 3rd-person perspective.
	pub fn draw_third_person(&self, ctx: &GLContext) {
		let (shader, meshes) = self.draw_setup(ctx);
		self.draw_head(shader, meshes);
		self.draw_feet(shader, meshes);
	}

	/// Partially draw this model, as seen from a 1rd-person perspective.
	pub fn draw_first_person(&self, ctx: &GLContext) {
		let (shader, meshes) = self.draw_setup(ctx);
		self.draw_feet(shader, meshes);
	}

	fn draw_setup<'a>(&self, ctx: &'a GLContext) -> (&'a AnimShader, &'a MeshPack) {
		ctx.set_depth_test(true);
		let shader = ctx.shaders().bind_anim_shader();
		ctx.textures().bind_skins();
		shader.set_texture(self.skin_tex);
		(shader, ctx.meshes())
	}

	fn draw_head(&self, shader: &AnimShader, meshes: &MeshPack) {
		let head = self.head_pos_internal();
		let head_pitch = self.pitch / 2.0; // reduced head pitch looks less silly
		shader.set_transform(head_pitch, head, self.yaw, self.pos);
		meshes.heads[self.head_mesh].bind_and_draw();
	}

	fn head_pos_internal(&self) -> vec3 {
		vec3(0.0, self.vsize - 0.5 * self.hsize, 0.0)
	}

	fn draw_feet(&self, shader: &AnimShader, meshes: &MeshPack) {
		let (foot1, foot2) = self.feet_pos_internal();
		shader.set_transform(0.0, foot1, self.yaw, self.pos);
		meshes.feet[self.foot_mesh].bind_and_draw();
		shader.set_transform(0.0, foot2, self.yaw, self.pos);
		meshes.feet[self.foot_mesh].bind_and_draw();
	}

	fn feet_pos_internal(&self) -> (vec3, vec3) {
		let anim_r = 0.2;
		let c = anim_r * self.feet_phase.cos();
		let s = anim_r * self.feet_phase.sin();
		(
			vec3(-0.35 * self.hsize, f32::max(0.0, s), c),  // left
			vec3(0.35 * self.hsize, f32::max(0.0, -s), -c), // right
		)
	}

	/// Unit vector in the player's looking direction.
	pub fn look_dir(&self) -> vec3 {
		let yaw = self.yaw as f32;
		let pitch = self.pitch as f32;
		let x = -f32::sin(yaw) * f32::cos(-pitch);
		let z = -f32::cos(yaw) * f32::cos(-pitch);
		let y = f32::sin(-pitch);
		vec3(x, y, z)
	}

	pub fn look_dir_h(&self) -> vec3 {
		let yaw = self.yaw as f32;
		let x = -f32::sin(yaw);
		let z = -f32::cos(yaw);
		let y = 0.0;
		vec3(x, y, z)
	}

	/// Direction right of look_dir
	pub fn look_right(&self) -> vec3 {
		let look = self.look_dir();
		vec3(-look.z, 0.0, look.x).normalized()
	}

	/// Rotate the view direction.
	pub fn rotate(&mut self, delta_yaw: f32, delta_pitch: f32) {
		self.set_yaw(self.yaw + delta_yaw);
		self.set_pitch(self.pitch + delta_pitch);
	}

	pub fn set_yaw(&mut self, yaw: f32) {
		// wrap around yaw so that it stays nicely witin -PI..PI.
		self.yaw = wrap_angle(yaw);
	}

	pub fn set_pitch(&mut self, pitch: f32) {
		// clamp pitch to +/- 90 degrees,
		// so that we can't look backwards, upside down.
		self.pitch = clamp(pitch, -PI / 2.0, PI / 2.0);
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn look_dir() {
		let mut s = Model::new(1.0, 1.0, 0, 0, 0);
		s.yaw = 0.0;
		assert_eq!(s.look_dir(), vec3(0.0, 0.0, -1.0));
		s.yaw = 90.0 * DEG;
		assert_eq!(s.look_dir().map(|v| v.round()), vec3(-1.0, 0.0, 0.0));
		s.yaw = -90.0 * DEG;
		assert_eq!(s.look_dir().map(|v| v.round()), vec3(1.0, 0.0, 0.0));
		s.yaw = 180.0 * DEG;
		assert_eq!(s.look_dir().map(|v| v.round()), vec3(0.0, 0.0, 1.0));
	}
}
