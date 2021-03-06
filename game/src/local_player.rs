use super::prelude::*;

/// Controls a Player via local keyboard/mouse input
/// (as opposed to `Bot`, which controls a player autonomously).
pub struct LocalPlayer {
	client: Client,

	// record key presses and mouse movements in between ticks (redraws).
	keys: KeyStates,

	can_bunny_hop: bool,

	jump_armed: bool,
}

impl LocalPlayer {
	pub fn new(client: Client) -> Self {
		Self {
			client,
			keys: KeyStates::new(),
			can_bunny_hop: false,
			jump_armed: true,
		}
	}

	/// Record that `key` was pressed or released.
	pub fn record_key(&mut self, key: Key, pressed: bool) {
		self.keys.record(key, pressed)
	}

	/// Record that the mouse was moved by `(delta_x, delta_y)`.
	pub fn record_mouse(&mut self, delta: (f64, f64)) {
		self.keys.record_mouse(delta)
	}

	/// Advance time, controlling the player based on the key presses
	/// recorded since the last call to `tick`.
	pub fn tick(&mut self, dt: f32) {
		// catch up with the server state
		self.client.tick(dt);

		// control a copy of the player (not allowed to mutate game state in-place)
		// record changes to the player and Map as update Messages.
		let mut player = self.player().clone();
		let mut updates = Updates::new();
		self.control(&mut player, dt, &mut updates);

		// send update messages back to the server
		self.client.send_updates(updates);
		self.client.update_player(player);

		self.keys.clear(); // must be last
	}

	/// Control the player over interval of `dt` seconds.
	/// Record changes to map in `updates`.
	/// Player state (position, etc) is updated in-place.
	fn control(&mut self, player: &mut Player, dt: f32, updates: &mut Updates) {
		self.control_look_dir(player);
		self.control_weapon(player, dt, updates);
		self.control_movement(player, dt);
	}

	/// Control the player's look direction,
	/// based on mouse movements since the last call to `control`.
	fn control_look_dir(&mut self, player: &mut Player) {
		let (dx, dy) = self.keys.mouse_delta();
		player.rotate(-dx, dy); // positive yaw = CCW
	}

	fn control_weapon(&mut self, player: &mut Player, dt: f32, updates: &mut Updates) {
		// select different weapon?
		for (i, &k) in Key::NUMERIC_KEYS.iter().take(player.weapons.len()).enumerate() {
			if self.keys.was_pressed(k) {
				player.selected_weapon = i;
			}
		}

		// fire?
		if self.keys.is_down(Key::Mouse1) {
			player.fire_weapon(dt, false /*alt*/, self.gs(), updates);
		}

		// alt fire?
		if self.keys.is_down(Key::Mouse3) {
			player.fire_weapon(dt, true /*alt*/, self.gs(), updates);
		}
	}

	fn control_movement(&mut self, player: &mut Player, dt: f32) {
		// jump
		let on_ground = player.model.on_ground(self.map());

		if on_ground && (!self.keys.is_down(Key::Jump) || self.can_bunny_hop) {
			self.jump_armed = true;
		}
		if self.keys.is_down(Key::Jump) && on_ground && self.jump_armed {
			self.jump_armed = false;
			player.jump(self.gs());
		}

		// walk/fly
		let walk_dir = Self::walk_dir(player, &self.keys);
		let sprint = self.keys.is_down(Key::Sprint);
		player.walk(dt, walk_dir, sprint, self.gs());

		//let walk_speed = if self.keys.is_down(Key::Sprint) { Player::SPRINT_SPEED } else { Player::WALK_SPEED };
		//player.model.try_walk(dt, self.map(), walk_speed * walk_dir);

		player.model.tick(dt, self.map());
	}

	/// Direction the player wants to move in,
	/// based on the currently pressed keys and look direction.
	fn walk_dir(player: &Player, keys: &KeyStates) -> vec3 {
		let mut dir = vec3::ZERO;
		if keys.is_down(Key::Left) {
			dir.x -= 1.0;
		}
		if keys.is_down(Key::Right) {
			dir.x += 1.0;
		}
		if keys.is_down(Key::Forward) {
			dir.z -= 1.0;
		}
		if keys.is_down(Key::Backward) {
			dir.z += 1.0;
		}
		if dir == vec3::ZERO {
			return vec3::ZERO;
		}
		let dir = yaw_matrix(-player.model.yaw as f32).transform_point_ignore_w(dir);
		dir.safe_normalized()
	}

	pub fn player(&self) -> &Player {
		self.client.player()
	}

	pub fn gs(&self) -> &GameState {
		self.client.game_state()
	}

	pub fn map(&self) -> &Map {
		&self.gs().map
	}

	pub fn draw(&self, ctx: &GLContext) {
		self.client.draw(ctx)
	}
}
