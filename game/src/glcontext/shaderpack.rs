use super::internal::*;

/// A collection of shader programs (linked vertex + fragment shaders)
/// that all work with Mesh vaos.
pub struct ShaderPack {
	flat_shader_2d: FlatShader,
	flat_shader_3d: FlatShader,
	voxel_shader: VoxelShader,
	anim_shader: AnimShader,
}

impl ShaderPack {
	pub const VERTEX_POS: u32 = 1; // vertex shader
	pub const TEX_COORD: u32 = 2; // vertex shader
	pub const LIGHT: u32 = 5; //vertex shader
	const MATRIX: u32 = 4; // vertex shader
	const FLAT_COLOR: u32 = 5; // flat.frag
	const TEXTURE: u32 = 6; // voxel.frag, anim.frag
	const INV_VIEW_DIST_SQ: u32 = 7; // voxel.frag, anim.frag
	const SUN_DIR: u32 = 8; // amin.frag
	const VIEW_POS: u32 = 10; // anim.vert
	const FOG_COLOR: u32 = 11; // anim.frag, voxel.frag

	pub fn new() -> Self {
		ShaderPack {
			flat_shader_2d: FlatShader::new(),
			flat_shader_3d: FlatShader::new(),
			voxel_shader: VoxelShader::new(),
			anim_shader: AnimShader::new(),
		}
	}

	/// Sets the fog distance for all relevant shaders.
	pub fn set_fog_dist(&self, dist: f32) {
		let v = 1.0 / (dist * dist);
		self.voxel_shader.prog.uniform1f(Self::INV_VIEW_DIST_SQ, v);
		self.anim_shader.prog.uniform1f(Self::INV_VIEW_DIST_SQ, v);
	}

	/// Sets the fog color for all relevant shaders.
	pub fn set_fog_color(&self, color: vec3) {
		self.voxel_shader.prog.uniform3f(Self::FOG_COLOR, color.x, color.y, color.z);
		self.anim_shader.prog.uniform3f(Self::FOG_COLOR, color.x, color.y, color.z);
	}

	/// Sets the sunlight direction for all relevant shaders.
	pub fn set_sun_dir(&self, dir: vec3) {
		self.anim_shader.prog.uniform3f(Self::SUN_DIR, dir.x, dir.y, dir.z);
	}

	/// Sets the projection matrices / viewports / view positions of all shaders.
	pub fn set_matrix(&self, viewport: (u32, u32), (pos, yaw, pitch): (vec3, f32, f32)) {
		let proj = camera_matrix(viewport, pos, yaw, pitch);
		let proj_array = proj.as_array();
		let iso = isometric_matrix(viewport);

		self.flat_shader_2d.prog.uniform_matrix4f(ShaderPack::MATRIX, false /*transpose*/, iso.as_array());
		self.flat_shader_3d.prog.uniform_matrix4f(ShaderPack::MATRIX, false /*transpose*/, proj_array);
		self.anim_shader.prog.uniform_matrix4f(ShaderPack::MATRIX, false /*transpose*/, proj_array);
		self.voxel_shader.prog.uniform_matrix4f(ShaderPack::MATRIX, false /*transpose*/, proj_array);

		// Used for fog, specular reflections:
		self.anim_shader.prog.uniform3f(Self::VIEW_POS, pos.x, pos.y, pos.z);
		self.voxel_shader.prog.uniform3f(Self::VIEW_POS, pos.x, pos.y, pos.z);
	}

	/// Bind and return a shader with isometric projection and flat colors.
	/// Intended to draw the crosshair.
	pub fn bind_flat_shader_2d(&self) -> &FlatShader {
		self.flat_shader_2d.bind();
		self.flat_shader_2d.reset();
		&self.flat_shader_2d
	}

	/// Bind and return a shader with camera projection and flat colors.
	pub fn bind_flat_shader_3d(&self) -> &FlatShader {
		self.flat_shader_3d.bind();
		self.flat_shader_3d.reset();
		&self.flat_shader_3d
	}

	/// Bind and return the voxel shader.
	pub fn bind_voxel_shader(&self) -> &VoxelShader {
		self.voxel_shader.bind();
		self.voxel_shader.reset();
		&self.voxel_shader
	}

	/// Bind and return the animation shader.
	pub fn bind_anim_shader(&self) -> &AnimShader {
		self.anim_shader.prog.use_program();
		self.anim_shader.reset();
		&self.anim_shader
	}
}

/// Shader and textures for drawing animated models.
pub struct AnimShader {
	prog: Program,
}

impl AnimShader {
	const YAW: u32 = 9; // anim.vert
	const EXT_TRANSLATION: u32 = 3; // anim.vert
	const INT_TRANSLATION: u32 = 12; // anim.vert
	const PITCH: u32 = 13; // anim.vert

	/// AnimShader with textures loaded from a directory.
	pub fn new() -> Self {
		let vertex_shader = Shader::new_vertex(include_str!("shaders/anim.vert"));
		let texture_shader = Shader::new_fragment(include_str!("shaders/anim.frag"));
		Self {
			prog: Program::new(&[&vertex_shader, &texture_shader]),
		}
	}

	pub fn set_texture(&self, texid: usize) {
		self.prog.uniform1i(ShaderPack::TEXTURE, texid as i32)
	}

	/// Set vertex transformation to:
	///
	///  pitch
	///  + translate + yaw
	///  + translate
	///  + projection (set globally on parent GLContext)
	///
	pub fn set_transform(&self, pitch: f32, int_translate: vec3, yaw: f32, ext_translate: vec3) {
		self.prog.uniform1f(Self::PITCH, pitch);
		self.prog.uniform3f(Self::INT_TRANSLATION, int_translate.x, int_translate.y, int_translate.z);
		self.prog.uniform1f(Self::YAW, yaw);
		self.prog.uniform3f(Self::EXT_TRANSLATION, ext_translate.x, ext_translate.y, ext_translate.z);
	}

	pub fn reset(&self) {
		self.set_texture(0);
		self.set_transform(0.0, vec3::ZERO, 0.0, vec3::ZERO);
	}
}

/// Shaders + textures for drawing Voxels (cubes).
pub struct VoxelShader {
	prog: Program,
}

impl VoxelShader {
	/// VoxelShader with textures loaded from a directory.
	pub fn new() -> Self {
		let vertex_shader = Shader::new_vertex(include_str!("shaders/voxel.vert"));
		let texture_shader = Shader::new_fragment(include_str!("shaders/voxel.frag"));
		Self {
			prog: Program::new(&[&vertex_shader, &texture_shader]),
		}
	}

	fn bind(&self) {
		self.prog.use_program();
		self.set_texture(0);
	}

	pub fn set_texture(&self, texid: usize) {
		self.prog.uniform1i(ShaderPack::TEXTURE, texid as i32)
	}

	pub fn reset(&self) {
		self.set_texture(0);
	}
}

pub struct FlatShader {
	prog: Program,
}

impl FlatShader {
	pub fn new() -> Self {
		let vertex_shader = Shader::new_vertex(include_str!("shaders/flat.vert"));
		let texture_shader = Shader::new_fragment(include_str!("shaders/flat.frag"));
		Self {
			prog: Program::new(&[&vertex_shader, &texture_shader]),
		}
	}

	fn bind(&self) {
		self.prog.use_program();
		self.set_color(vec3::ZERO); // clear previous state
	}

	pub fn set_color(&self, col: vec3) {
		self.set_color_alpha(col, 1.0)
	}

	pub fn set_color_alpha(&self, col: vec3, alpha: f32) {
		self.prog.uniform4f(ShaderPack::FLAT_COLOR, col.x, col.y, col.z, alpha)
	}

	pub fn reset(&self) {
		self.set_color(vec3::ZERO);
	}
}
