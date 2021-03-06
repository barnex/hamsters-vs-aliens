use super::internal::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
	pub model: Model,
	pub weapons: [Weapon; 2],
	pub selected_weapon: usize,
}

impl Player {
	const H_SIZE: f32 = 0.8;
	const V_SIZE: f32 = 1.9;
	const CAM_HEIGHT: f32 = Self::V_SIZE - 0.05;
	pub const WALK_SPEED: f32 = 6.0;
	pub const JUMP_SPEED: f32 = 9.0;
	pub const SPRINT_SPEED: f32 = 9.0; // Used by Model stairclimb. TODO: remove dependency

	pub fn new(skin: usize) -> Self {
		let model = Model::new(Self::H_SIZE, Self::V_SIZE, skin, skin, skin);
		Self {
			model,
			weapons: [Weapon::laser(), Weapon::snow_cannon()],
			selected_weapon: 0,
		}
	}

	/// Center-bottom position of the player's bounding box (world coordinates).
	pub fn bottom_pos(&self) -> vec3 {
		self.model.pos
	}

	/// Player's center position (world coordinates).
	pub fn center(&self) -> vec3 {
		self.model.pos + vec3(0.0, Self::H_SIZE / 2.0, 0.0)
	}

	pub fn look_dir(&self) -> vec3 {
		self.model.look_dir()
	}

	// ___________________________________ movement

	pub fn jump(&mut self, gs: &GameState) {
		if self.model.on_ground(&gs.map) {
			self.model.vel.y = Self::JUMP_SPEED;
		}
	}

	pub fn walk(&mut self, dt: f32, dir: vec3, sprint: bool, gs: &GameState) {
		let speed = if sprint { Self::SPRINT_SPEED } else { Self::WALK_SPEED };
		self.model.try_walk(dt, gs.map(), speed * dir.safe_normalized());
	}

	/// Rotate the player's view direction by `yaw`, `pitch` radians.
	pub fn rotate(&mut self, yaw: f32, pitch: f32) {
		self.model.rotate(yaw, pitch)
	}

	// ___________________________________ shoot

	pub fn fire_weapon(&mut self, dt: f32, alt: bool, gs: &GameState, updates: &mut Updates) {
		let orient = self.weapon_orientation();
		self.weapons[self.selected_weapon].fire(dt, alt, orient, gs, updates)
	}

	fn weapon_orientation(&self) -> WeaponOrientation {
		(self.weapon_pos_abs(), self.view_pos(), self.look_dir())
	}

	fn selected_weapon(&self) -> &Weapon {
		&self.weapons[self.selected_weapon]
	}

	const WEAPON_POS_INTERNAL: vec3 = vec3(0.5, 1.5, 0.0);

	// Weapon position (world coordinates).
	pub fn weapon_pos_abs(&self) -> vec3 {
		let rel = Self::WEAPON_POS_INTERNAL;
		self.bottom_pos() + (rel.x * self.model.look_right()) + vec3(0.0, rel.y, 0.0)
		//vec3(, self.model.vsize * 0.7, 0.0) + WEAPON_DIST * self.model.look_right()
	}

	fn weapon_pos_internal(&self) -> vec3 {
		Self::WEAPON_POS_INTERNAL
	}

	// _____________________________________ draw

	/// Camera position (world coordinates), yaw, pitch (radians).
	pub fn camera(&self) -> (vec3, f32, f32) {
		(self.view_pos(), self.model.yaw as f32, self.model.pitch as f32)
	}

	/// Position of the player's eye (world coordinates).
	pub fn view_pos(&self) -> vec3 {
		self.bottom_pos() + Self::CAM_HEIGHT * vec3::EY
	}

	/// Draw the player from a 3rd-person perspective.
	/// I.e.: draw the entire model, but not the crosshair.
	pub fn draw_third_person(&self, ctx: &GLContext) {
		self.model.draw_third_person(ctx);
		self.draw_weapon(ctx);
	}

	/// Draw the player from a 1st-person perspective.
	/// I.e.: draw the player-visible part of the model, and the weapon and crosshair.
	pub fn draw_first_person(&self, ctx: &GLContext) {
		self.model.draw_first_person(ctx);
		self.draw_weapon(ctx);
	}

	fn draw_weapon(&self, ctx: &GLContext) {
		self.selected_weapon().draw(ctx, (self.model.pitch, self.weapon_pos_internal(), self.model.yaw, self.model.pos));
	}
}
