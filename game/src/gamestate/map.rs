use super::internal::*;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};

/// A world, without connected players.
/// Maps are saved as `.json.gz` files.
#[derive(Serialize, Deserialize)]
pub struct Map {
	voxels: VoxelBox,
	pub fog_dist: f32,
	pub background_color: vec3,
	pub sun_dir: vec3,
}

impl Map {
	const DEFAULT_BACKGROUND_COLOR: vec3 = vec3(0.95, 0.90, 0.85);
	const DEFAULT_FOG_DIST: f32 = 1e2;
	const DEFAULT_SUN_DIR: vec3 = vec3(0.2721, 0.9525, 0.1360);
	//const VIEW_DIST: f32 = 150.0;

	/// An empty map with given size
	/// and default environmental parameters.
	pub fn new(size: uvec3) -> Self {
		Self {
			voxels: VoxelBox::new(size),
			background_color: Self::DEFAULT_BACKGROUND_COLOR,
			fog_dist: Self::DEFAULT_FOG_DIST,
			sun_dir: Self::DEFAULT_SUN_DIR,
		}
	}

	/// A square map populated with a flat, snowy surface.
	pub fn flat(size: uvec3) -> Self {
		let mut map = Map::new(size);
		let (nx, _, nz) = map.size().as_ivec().into();
		for x in 0..nx {
			for z in 0..nz {
				map.set(ivec3(x, 0, z), Voxel::SNOW)
			}
		}
		map
	}

	// _________________________ accessors ______________________

	pub fn size(&self) -> uvec3 {
		self.voxels.world_size()
	}

	pub fn at(&self, index: ivec3) -> Voxel {
		self.voxels.at(index)
	}

	pub fn at_pos(&self, pos: vec3) -> Voxel {
		self.at(Self::voxel_index(pos))
	}

	pub fn set(&mut self, index: ivec3, v: Voxel) {
		self.voxels.set(index, v)
	}

	pub fn voxel_index(pos: vec3) -> ivec3 {
		pos.to_ivec()
	}

	pub fn bumps(&self, bounds: &BoundingBox) -> bool {
		self.voxels.bumps(bounds)
	}

	pub fn intersect(&self, start: vec3, dir: vec3, max: f32, bw_offset: bool) -> Option<vec3> {
		let step = 0.1; // ray march with this step size
		let num_steps = (max / step) as usize + 1;
		let mut t = 0.0;

		for _i in 0..num_steps {
			t += step;
			let probe = start + t * dir;
			if self.at_pos(probe) != Voxel::EMPTY {
				if bw_offset {
					return Some(start + (t - step) * dir);
				} else {
					return Some(probe);
				}
			}
		}
		None
	}

	pub fn intersects(&self, start: vec3, dir: vec3, max: f32) -> bool {
		self.intersect(start, dir, max, false) != None
	}

	// _______________________ I/O ___________________________

	/// Serialize in gzipped JSON.
	pub fn serialize<W: Write>(&self, w: W) -> Result<()> {
		let gz = GzEncoder::new(w, Compression::best());
		Ok(serde_json::to_writer(gz, self)?)
	}

	/// Deserialize from gzipped JSON.
	pub fn deserialize<R: Read>(r: R) -> Result<Self> {
		let gz = GzDecoder::new(r);
		Ok(serde_json::from_reader(gz)?)
	}

	/// Serialize to file, `.json.gz`
	pub fn save<P: AsRef<Path>>(&self, fname: P) -> Result<()> {
		self.serialize(BufWriter::new(File::create(fname)?))
	}

	/// Deserialize from file, `.json.gz`
	pub fn load<P: AsRef<Path>>(fname: P) -> Result<Self> {
		Self::deserialize(BufReader::new(File::open(fname)?))
	}

	/// Serialize to bytes, gzipped JSON.
	pub fn to_bytes(&self) -> Vec<u8> {
		let mut buf = Vec::new();
		self.serialize(&mut buf).unwrap();
		buf
	}

	/// Deserialize from bytes, gzipped JSON.
	pub fn from_bytes(map_data: &[u8]) -> Result<Self> {
		Self::deserialize(std::io::Cursor::new(map_data))
	}

	// __________________________ draw __________________________

	pub fn draw(&self, ctx: &GLContext, view_pos: vec3) {
		ctx.clear(self.background_color);
		ctx.shaders().set_fog_dist(self.fog_dist);
		ctx.shaders().set_fog_color(self.background_color);
		ctx.shaders().set_sun_dir(self.sun_dir);
		self.voxels.draw(ctx, view_pos, self.fog_dist);
	}
}
