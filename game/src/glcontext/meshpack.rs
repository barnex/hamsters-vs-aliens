use super::internal::*;
use crate::wavefrontobj;

pub struct MeshPack {
	pub crosshair: Mesh,
	pub heads: Vec<Mesh>,
	pub feet: Vec<Mesh>,
	pub particles: Vec<Mesh>,
	pub weapon: Mesh,
}

impl MeshPack {
	pub fn new(mesh_dir: &Path) -> Self {
		Self {
			crosshair: Self::crosshair(),

			heads: vec![
				Self::load(mesh_dir, "xyz.obj", 1.0),         //
				Self::load(mesh_dir, "hamsterhead.obj", 1.2), //
				Self::load(mesh_dir, "froghead2.obj", 1.0),   //
				Self::load(mesh_dir, "chickenhead.obj", 1.0), //
				Self::load(mesh_dir, "head0.obj", 1.0),       //
				Self::load(mesh_dir, "tadpolehead.obj", 1.0), //
			],
			feet: vec![
				Self::load(mesh_dir, "foot0.obj", 0.45),       //
				Self::load(mesh_dir, "hamsterfoot.obj", 0.45), //
				Self::load(mesh_dir, "frogfoot.obj", 0.64),    //
				Self::load(mesh_dir, "chickenleg.obj", 1.0),   //
				Self::load(mesh_dir, "foot1.obj", 0.65),       //
			],
			particles: vec![Self::load(mesh_dir, "cube.obj", 1.0)], // TODO
			weapon: Self::load(mesh_dir, "gun2.obj", 1.5),
		}
	}

	fn load(mesh_dir: &Path, base: &str, scale: f32) -> Mesh {
		let fname = mesh_dir.join(base);
		let obj = match wavefrontobj::parse_file(&fname) {
			Ok(obj) => obj,
			Err(e) => {
				eprintln!("load {}: {}", fname.to_string_lossy(), e);
				MeshBuffer::cube()
			}
		};
		obj.scale(scale).build()
	}

	fn crosshair() -> Mesh {
		let c = 0.004; //crosshair size
		let z = 0.5;
		let v = [
			c * vec3(-1.0, 0.0, z), //
			c * vec3(-0.2, 0.0, z),
			c * vec3(0.2, 0.0, z),
			c * vec3(1.0, 0.0, z), //
			c * vec3(0.0, -1.0, z),
			c * vec3(0.0, -0.2, z),
			c * vec3(0.0, 0.2, z),
			c * vec3(0.0, 1.0, z),
		];
		Mesh::new(&v)
	}
}
