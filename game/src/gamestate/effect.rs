pub use super::internal::*;

// TODO: don't serialize. Send Map, Players only.
#[derive(Serialize, Deserialize, Clone)]
pub struct Effects {
	inner: Vec<Effect>,
}

impl Effects {
	pub fn new() -> Self {
		Self { inner: Vec::new() }
	}

	pub fn push(&mut self, e: Effect) {
		self.inner.push(e)
	}

	pub fn draw(&self, ctx: &GLContext) {
		ctx.set_depth_test(true);
		ctx.textures().bind_voxels();

		// TODO: observe view distance
		for effect in &self.inner {
			effect.draw(ctx)
		}
	}

	pub fn tick(&mut self, dt: f32) {
		for p in &mut self.inner {
			p.tick(dt)
		}
		self.prune_dead();
	}

	// remove dead effects.
	fn prune_dead(&mut self) {
		let mut i = 0;
		loop {
			if i >= self.inner.len() {
				break;
			}
			if !self.inner[i].alive() {
				self.inner.swap_remove(i);
			} else {
				i += 1;
			}
		}
	}
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Effect {
	ParticleEffect(Particle),
	TrailEffect(Trail),
}
use Effect::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct Trail {
	ttl: f32,
	start: vec3,
	stop: vec3,
	color: vec3,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Particle {
	pub ttl: f32,
	pub pos: vec3,
	pub vel: vec3,
	pub acc: f32,
	pub mesh: u8,
	pub tex: u8,
}

impl Effect {
	pub fn tick(&mut self, dt: f32) {
		match self {
			ParticleEffect(e) => e.tick(dt),
			TrailEffect(e) => e.tick(dt),
		}
	}

	pub fn alive(&self) -> bool {
		match self {
			ParticleEffect(e) => e.ttl > 0.0,
			TrailEffect(e) => e.ttl > 0.0,
		}
	}

	pub fn draw(&self, ctx: &GLContext) {
		// TODO: limit shader switches (expensive)
		match self {
			ParticleEffect(e) => e.draw(ctx),
			TrailEffect(e) => e.draw(ctx),
		}
	}
}

impl Trail {
	const TTL: f32 = 0.2;

	pub fn new(start: vec3, stop: vec3, color: vec3) -> Self {
		Self { ttl: Self::TTL, start, stop, color }
	}

	fn tick(&mut self, dt: f32) {
		self.ttl -= dt;
	}

	fn draw(&self, ctx: &GLContext) {
		ctx.set_rel_line_width(0.005);
		let shader = ctx.shaders().bind_flat_shader_3d();

		let alpha = self.ttl / Self::TTL;
		shader.set_color_alpha(self.color, alpha);

		let vert = [self.start, self.stop];
		Mesh::new(&vert).bind_and_draw_mode(gl::LINES);
	}
}

impl Particle {
	fn tick(&mut self, dt: f32) {
		self.ttl -= dt;

		self.vel.y += self.acc * dt;
		self.pos += self.vel * dt;
	}

	fn draw(&self, ctx: &GLContext) {
		let shader = ctx.shaders().bind_anim_shader(); // TODO: limit switches
		shader.set_texture(self.tex as usize);
		shader.set_transform(0.0, vec3::ZERO, 0.0, self.pos);
		ctx.meshes().particles[self.mesh as usize].bind_and_draw();
	}
}
