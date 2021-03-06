use super::prelude::*;

#[derive(Clone)]
pub struct Vertex {
	pub pos: vec3,
	pub texcoord: vec2,
	pub attrib: vec3,
}

impl Vertex {
	pub fn new(pos: vec3, texcoord: vec2, attrib: vec3) -> Self {
		Self { pos, texcoord, attrib }
	}
}
