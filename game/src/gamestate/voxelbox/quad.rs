use super::internal::*;

#[derive(Clone)]
pub struct Quad {
	vertices: [Vertex; 4],
	center: Vertex,
}

impl Quad {
	pub fn new(v0: Vertex, v1: Vertex, v2: Vertex, v3: Vertex) -> Self {
		let center = Vertex {
			pos: (v0.pos + v1.pos + v2.pos + v3.pos) / 4.0,
			texcoord: (v0.texcoord + v1.texcoord + v2.texcoord + v3.texcoord) / 4.0,
			attrib: (v0.attrib + v1.attrib + v2.attrib + v3.attrib) / 4.0,
		};
		Self { vertices: [v0, v1, v2, v3], center }
	}

	pub fn normal(&self) -> vec3 {
		self.tangent2().cross(self.tangent1())
	}

	pub fn tangent1(&self) -> vec3 {
		self.vertices[1].pos - self.vertices[0].pos
	}

	pub fn tangent2(&self) -> vec3 {
		self.vertices[3].pos - self.vertices[0].pos
	}

	pub fn triangle_vertices(&self) -> [&Vertex; 12] {
		[
			&self.center,
			&self.vertices[1],
			&self.vertices[0], // triangle 1
			&self.center,
			&self.vertices[2],
			&self.vertices[1], // triangle 2
			&self.center,
			&self.vertices[3],
			&self.vertices[2], // triangle 3
			&self.center,
			&self.vertices[0],
			&self.vertices[3], // triangle 4
		]
	}

	// TODO: remove
	pub fn set_vertex(&mut self, i: usize, v: Vertex) {
		self.vertices[i] = v;
		let v = &self.vertices;
		self.center = Vertex {
			pos: (v[0].pos + v[1].pos + v[2].pos + v[3].pos) / 4.0,
			texcoord: (v[0].texcoord + v[1].texcoord + v[2].texcoord + v[3].texcoord) / 4.0,
			attrib: (v[0].attrib + v[1].attrib + v[2].attrib + v[3].attrib) / 4.0,
		};
	}

	pub fn vertex(&self, i: usize) -> &Vertex {
		&self.vertices[i]
	}
}
