use super::prelude::*;

/// Representation of a mesh (vertex array with attributes that work with ShaderPack),
/// in CPU (host) memory -- ready to be transferred to the GPU.
#[derive(Clone, Default)]
pub struct MeshBuffer {
	vertices: Vec<vec3>,
	texcoords: Vec<vec2>,
	vertex_attribs: Vec<vec3>,
}

impl MeshBuffer {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn cube() -> Self {
		let mut s = Self::new();
		for quad in &super::voxelbox::cube::cube_at(vec3(-0.5, -0.5, -0.5)) {
			s.push_all(&quad.triangle_vertices());
		}
		s
	}

	pub fn push(&mut self, v: &Vertex) {
		self.vertices.push(v.pos);
		self.texcoords.push(v.texcoord);
		self.vertex_attribs.push(v.attrib);
	}

	pub fn push_all(&mut self, v: &[&Vertex]) {
		for v in v {
			self.push(v);
		}
	}

	/// Scale all vertex positions.
	/// Handy when an obj file does not have the right size.
	pub fn scale(mut self, scale: f32) -> Self {
		for v in &mut self.vertices {
			*v = *v * scale;
		}
		self
	}

	pub fn build(&self) -> Mesh {
		Mesh::new(&self.vertices) //
			.with_tex_coords(&self.texcoords)
			.with_vertex_attribs(&self.vertex_attribs)
	}

	pub fn len(&self) -> usize {
		self.vertices.len()
	}

	pub fn vertex_positions(&self) -> &[vec3] {
		&self.vertices
	}
}
