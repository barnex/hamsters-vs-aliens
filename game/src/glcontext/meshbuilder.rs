use super::internal::*;

#[derive(Clone, Default)]
pub struct MeshBuilder {
	vertices: Vec<vec3>,
	texcoords: Vec<vec2>,
	vertex_attribs: Vec<vec3>,
}

impl MeshBuilder {
	pub fn new() -> Self {
		Self::default()
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

	pub fn build(&self) -> Mesh {
		Mesh::new(&self.vertices) //
			.with_tex_coords(&self.texcoords)
			.with_vertex_attribs(&self.vertex_attribs)
	}

	pub fn len(&self) -> usize {
		self.vertices.len()
	}
}
