use super::internal::*;
use gl;
use gl::types::*;
use gl_safe::*;

pub struct Mesh {
	vao: VertexArray, // guaranteed to comply with the attribute layout expected by ShaderPack.
}

impl Mesh {
	//TODO: 2D texcoords, need normals?

	pub fn vao(&self) -> &VertexArray {
		&self.vao
	}

	pub fn new(vertices: &[vec3]) -> Self {
		let v_pos_buf = Buffer::create().storage(&vertices, 0);
		let vao = VertexArray::create()
			.enable_attrib(ShaderPack::VERTEX_POS)
			.attrib_format(ShaderPack::VERTEX_POS, 3, gl::FLOAT, false, 0)
			.vertex_buffer(ShaderPack::VERTEX_POS, v_pos_buf, 0, sizeof(vertices[0]));
		Self { vao }
	}

	/// Enable and set per-vertex texture coordinates.
	pub fn with_tex_coords(mut self, texcoords: &[vec2]) -> Self {
		debug_assert!(texcoords.len() == self.vao.len());
		let v_texc_buf = Buffer::create().storage(&texcoords, 0);
		self.vao = self
			.vao
			.enable_attrib(ShaderPack::TEX_COORD)
			.attrib_format(ShaderPack::TEX_COORD, 2, gl::FLOAT, false, 0)
			.vertex_buffer(ShaderPack::TEX_COORD, v_texc_buf, 0, sizeof(texcoords[0]));
		self
	}

	/// Enable and set per-vertex attribute (normal, light, ...) values.
	pub fn with_vertex_attribs(mut self, vertex_attribs: &[vec3]) -> Self {
		debug_assert!(vertex_attribs.len() == self.vao.len());
		let v_light_buf = Buffer::create().storage(&vertex_attribs, 0);
		self.vao =
			self.vao
				.enable_attrib(ShaderPack::LIGHT)
				.attrib_format(ShaderPack::LIGHT, 3, gl::FLOAT, false, 0)
				.vertex_buffer(ShaderPack::LIGHT, v_light_buf, 0, sizeof(vertex_attribs[0]));
		self
	}

	/// Bind this mesh and draw as TRIANGLES.
	pub fn bind_and_draw(&self) {
		self.bind_and_draw_mode(gl::TRIANGLES)
	}

	/// Bind this mesh and draw.
	pub fn bind_and_draw_mode(&self, mode: GLenum) {
		self.vao.bind();
		glDrawArrays(mode, 0 /*first*/, self.vao().len() as i32);
	}
}
