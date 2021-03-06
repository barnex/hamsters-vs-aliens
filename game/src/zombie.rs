/*
use super::prelude::*;
//use gl_safe::*;

pub struct Zombie {
	model: Model,
	speed: f32,
	destination: vec3,
}

impl Zombie {
	const H_SIZE: f32 = 0.9;
	const V_SIZE: f32 = 1.1;

	pub fn hamster(pos: vec3) -> Self {
		let mut model = Model::new(Self::H_SIZE, Self::V_SIZE, 1, 1, 1);
		model.pos = pos;
		Self {
			model,
			speed: 2.0,
			destination: pos + 10.0 * rand_vec(),
		}
	}

	pub fn frog(pos: vec3) -> Self {
		let mut model = Model::new(Self::H_SIZE, Self::V_SIZE, 2, 2, 2);
		model.pos = pos;
		Self {
			model,
			speed: 2.0,
			destination: pos + 10.0 * rand_vec(),
		}
	}

	pub fn draw(&self, ctx: &GLContext) {
		self.model.draw_third_person(ctx);
	}

	// advance time
	pub fn tick(&mut self, dt: f32, map: &mut Map, player: &Player) {
		if rand_every(dt, 5.0) {
			self.destination = player.pos() + 20.0 * rand_vec();
		}
		self.rotate_towards(dt, self.destination);

		let look_dir = self.model.look_dir();
		self.model.try_walk(dt, map, self.speed * look_dir);

		if rand_every(dt, 5.0) {
			self.model.try_jump(map, 8.0);
		}

		self.model.tick(dt, map);
	}

	const YAW_SPEED: f32 = 1.5; // rad / s

	fn rotate_towards(&mut self, dt: f32, dst: vec3) {
		let delta = dst - self.pos();
		let dst_angle = f32::atan2(-delta.x, -delta.z);
		let delta_angle = wrap_angle(dst_angle - self.model.yaw);
		self.model.set_yaw(self.model.yaw + dt * Self::YAW_SPEED * delta_angle.signum())
	}

	pub fn pos(&self) -> vec3 {
		self.model.pos
	}
}
*/
