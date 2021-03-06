use super::internal::*;
use std::thread::sleep;
use std::time::{Duration, Instant};

/// Controls a Player autonomously,
/// (as opposed to `LocalPlayer`, which controls a player by via keyboard/mouse input).
pub struct Bot {
	client: Client,
	time: Instant,
}

impl Bot {
	const MAX_LOOK_DIST: f32 = 100.0;

	// TODO: run(client);
	pub fn new(client: Client) -> Self {
		let (x, y, z) = client.game_state().map().size().into();
		println!("Hello {} x {} x {} world!", x, y, z);
		Self { client, time: Instant::now() }
	}

	// Update the Bot's local clock
	// and return elapsed seconds (`dt`) since last update
	fn update_time(&mut self) -> f32 {
		let now = Instant::now();
		let dt = (now - self.time).as_secs_f32();
		self.time = now;
		dt
	}

	/// Run loop autonomously controls the bot forever.
	/// Only returns in case of client network error.
	pub fn run_loop(&mut self) -> Result<()> {
		loop {
			sleep(Duration::from_millis(30));
			let dt = self.update_time();
			print!("\x1B[;H\x1B[J");
			self.tick(dt);
		}
	}

	// Advance time by `dt` second.
	fn tick(&mut self, dt: f32) {
		// catch up with latest server state
		self.client.tick(dt);

		// TODO: possible confusion between player and client.player()
		let mut me = self.client.player().clone();
		let mut updates = Updates::new();

		self.control(&mut me, dt, &mut updates);
		me.model.tick(dt, self.gs().map()); // TODO: tick(gs)

		self.client.update_player(me);
		self.client.send_updates(updates);
	}

	const SHOOT_CONE: f32 = 0.3;

	fn control(&mut self, me: &mut Player, dt: f32, updates: &mut Updates) {
		//find_target_player()

		if let Some(target) = self.closest_other_player() {
			let target = target - vec3(0.0, 2.0, 0.0); // shoot at feet :)
			println!("target: {:?}", target);
			self.rotate_towards(me, dt, target);

			let target_angle = angle_between(me.look_dir(), target - me.view_pos());
			println!("target_angle: {}", target_angle);
			if target_angle < Self::SHOOT_CONE {
				me.fire_weapon(dt, false /*alt*/, self.gs(), updates);
			}

			me.walk(dt, me.look_dir(), false, self.gs());
		} else {
			println!("target: None");
			println!("target_angle: None");

			me.jump(self.gs());
		}
	}

	const YAW_SPEED: f32 = 1.0; // rad / s

	fn rotate_towards(&mut self, me: &mut Player, dt: f32, target: vec3) {
		let delta = target - me.center();

		{
			let dst_yaw = f32::atan2(-delta.x, -delta.z);
			let needed_yaw = wrap_angle(dst_yaw - me.model.yaw);
			let delta_yaw = dt * Self::YAW_SPEED * needed_yaw.signum();
			if delta_yaw.abs() < needed_yaw.abs() {
				me.model.set_yaw(me.model.yaw + delta_yaw)
			}
		}

		let dst_pitch = f32::atan2(-delta.y, delta.xz().len());
		let needed_pitch = dst_pitch - me.model.pitch;
		let delta_pitch = dt * Self::YAW_SPEED * needed_pitch.signum();
		if delta_pitch.abs() < needed_pitch.abs() {
			me.model.set_pitch(me.model.pitch + delta_pitch)
		}
	}

	fn closest_other_player(&self) -> Option<vec3> {
		let me = self.client.player();

		let mut best_dist = INF;
		let mut pos = None;
		for (_, p) in self.client.other_players() {
			let dist = (p.center() - me.center()).len();
			if dist < best_dist && self.can_see(p.center()) {
				best_dist = dist;
				pos = Some(p.center());
			}
		}
		pos
	}

	//fn log(&self, msg: ??) {}

	fn can_see(&self, target: vec3) -> bool {
		let pos = self.client.player().view_pos();
		let delta = target - pos;
		if delta.len2() > (Self::MAX_LOOK_DIST * Self::MAX_LOOK_DIST) {
			return false;
		}

		//let look = self.client.player().model.look_dir();
		// TODO: look direction withing viewing cone.
		let blocked = self.gs().map().intersects(pos, delta.normalized(), delta.len());
		!blocked
	}

	fn gs(&self) -> &GameState {
		self.client.game_state()
	}
}

fn angle_between(a: vec3, b: vec3) -> f32 {
	a.normalized().dot(b.normalized()).acos()
}
