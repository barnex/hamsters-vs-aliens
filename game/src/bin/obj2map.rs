//! Convert wavefront object files into game maps.
use game::prelude::*;
use game::procedural_worlds;
use game::wavefrontobj;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
	/// Map size (voxels)
	#[structopt(short, long, default_value = "256")]
	pub size: u32,

	/// Wavefront Object file to open.
	pub obj_file: PathBuf,
}

fn main() -> Result<()> {
	let args = Args::from_args();

	let mesh = wavefrontobj::parse_file(&args.obj_file)?;
	let vertices = mesh.vertex_positions();
	let mesh_bb = BoundingBox::from(vertices.iter());
	let mesh_hsize = f32::max(mesh_bb.size().x, mesh_bb.size().z);
	println!("{} vertices", mesh.len());

	let mut map = Map::new(uvec3(args.size, args.size, args.size));
	let world_size = args.size as f32;

	let transform = |pos: vec3| ((pos / mesh_hsize) + vec3(0.5, 0.0, 0.5)) * world_size;

	let mut add_triangle = |v0: vec3, v1: vec3, v2: vec3| {
		let o = v0;
		let a = v1 - v0;
		let b = v2 - v0;

		let res = 0.3;
		let n_u = (a.len() / res) as usize + 1;
		let n_v = (b.len() / res) as usize + 1;

		for u in 0..(n_u + 1) {
			for v in 0..(n_v + 1) {
				let u = (u as f32) / (n_u as f32);
				let v = (v as f32) / (n_v as f32);

				if u + v > 1.0 {
					continue;
				}

				let pos = o + u * a + v * b;
				let ipos = pos.map(|v| (v + 0.5) as i32);
				map.set(ipos, Voxel::SNOW);
			}
		}
	};

	let mut i = 0;
	while i < vertices.len() {
		add_triangle(
			transform(vertices[i + 0]), //
			transform(vertices[i + 1]),
			transform(vertices[i + 2]),
		);

		i += 3;
	}

	procedural_worlds::floor(&mut map, 4, Voxel::LAVA);

	map.save("output.json.gz")
}

// fn max_size(vertices: &[vec3]) -> f32 {
// 	let mut size = 0.0;
// 	for pos in vertices {
// 		for v in &[pos.x, pos.y, pos.z] {
// 			if v.abs() > size {
// 				size = v.abs()
// 			}
// 		}
// 	}
// 	size
// }
