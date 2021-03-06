use super::internal::*;
use gl_safe::*;

pub struct GLContext {
	viewport: (u32, u32),
	shaders: ShaderPack,
	textures: TexturePack,
	meshes: MeshPack,
}

impl GLContext {
	pub fn new(texture_dir: &Path, mesh_dir: &Path) -> Self {
		Self {
			viewport: (0, 0),
			shaders: ShaderPack::new(),
			textures: TexturePack::new(texture_dir),
			meshes: MeshPack::new(mesh_dir),
		}
	}

	pub fn set_viewport(&mut self, viewport: (u32, u32)) {
		self.viewport = viewport;
		glViewport(0, 0, self.viewport.0 as i32, self.viewport.1 as i32);
	}

	pub fn set_matrix(&self, (pos, yaw, pitch): (vec3, f32, f32)) {
		self.shaders.set_matrix(self.viewport, (pos, yaw, pitch));
		//glViewport(0, 0, self.viewport.0 as i32, self.viewport.1 as i32);
	}

	pub fn shaders(&self) -> &ShaderPack {
		&self.shaders
	}

	pub fn meshes(&self) -> &MeshPack {
		&self.meshes
	}

	pub fn textures(&self) -> &TexturePack {
		&self.textures
	}

	pub fn clear(&self, color: vec3) {
		glClearColor(color.x, color.y, color.z, 1.0);
		glClear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
	}

	pub fn draw_crosshair(&self) {
		let ctx = self;
		ctx.set_depth_test(false);
		let shader = ctx.shaders().bind_flat_shader_2d();
		let crosshair = &ctx.meshes().crosshair;

		shader.set_color(BLACK);
		glLineWidth(2.5); //TODO: relative
		crosshair.bind_and_draw_mode(gl::LINES);

		shader.set_color(YELLOW);
		glLineWidth(2.0); //TODO: relative
		crosshair.bind_and_draw_mode(gl::LINES);

		ctx.set_depth_test(true);
	}

	pub fn set_depth_test(&self, enable: bool) {
		match enable {
			true => glEnable(gl::DEPTH_TEST),
			false => glDisable(gl::DEPTH_TEST),
		}
	}

	pub fn set_cull_face(&self, enable: bool) {
		match enable {
			true => glEnable(gl::CULL_FACE),
			false => glDisable(gl::CULL_FACE),
		}
	}

	pub fn set_rel_line_width(&self, rel_width: f32) {
		glLineWidth((self.viewport.0 as f32) * rel_width)
	}
}
