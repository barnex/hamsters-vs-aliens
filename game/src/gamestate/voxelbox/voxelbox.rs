use super::internal::*;
use std::cell::RefCell;

/// Sparse 3D array of Voxels.
pub struct VoxelBox {
	world_size: uvec3,
	chunks: Vec<Chunk>, // 2D array of chunks in XZ (horizontal)
	chunks_dim: uvec2,  // size of chunks array
}

/// A VoxelBox Chunk can grow/change independently of others,
/// and has its own Mesh.
pub struct Chunk {
	voxels: Vec<Voxel>,
	// A cache for this chunk's OpenGL Vertex Array.
	vaos: RefCell<Option<Vec<(usize, Mesh)>>>,
}

impl VoxelBox {
	/// New empty world with given size.
	/// X, Z must be a multiple of Chunck size.
	pub fn new(world_size: uvec3) -> Self {
		assert!(world_size.x != 0);
		assert!(world_size.y != 0);
		assert!(world_size.z != 0);
		assert!(world_size.x % Chunk::XZSIZE == 0);
		assert!(world_size.z % Chunk::XZSIZE == 0);

		let xchunks = world_size.x >> Chunk::POW;
		let zchunks = world_size.z >> Chunk::POW;
		let num_chunks = xchunks * zchunks;
		VoxelBox {
			world_size,
			chunks_dim: uvec2(xchunks, zchunks),
			chunks: zeros(num_chunks as usize),
		}
	}

	/// Return the Voxel at given position.
	/// Out-of-bounds access is safe, returns Voxel::EMPTY.
	pub fn at(&self, index: ivec3) -> Voxel {
		if !self.valid_index(index) {
			Voxel::EMPTY
		} else {
			let (chnk, int) = self.index_internal(index);
			self.chunk(chnk).at_internal(int)
		}
	}

	/// Set the Voxel at given position.
	/// Out-of-bounds positions are ignored.
	pub fn set(&mut self, index: ivec3, v: Voxel) {
		if !self.valid_index(index) {
			return;
		}
		let (chnk, int) = self.index_internal(index);
		self.chunk_mut(chnk).set_internal(int, v);

		// Invalidate this chunk's vao and those of it's neighbors
		// where the affected voxel was in or directly adjacent.
		// May invalidate the same chunk multiple times.
		for &dx in &[-1, 0, 1] {
			for &dy in &[-1, 0, 1] {
				let neigh_idx = index + ivec3(dx, 0, dy);
				if !self.valid_index(neigh_idx) {
					continue;
				}
				let (neigh_chnk, _) = self.index_internal(neigh_idx);
				self.invalidate_vao(neigh_chnk);
			}
		}
	}

	/// Tests if a boundingbox overlaps with ("bumps into") any non-empty voxel.
	pub fn bumps(&self, bounds: &BoundingBox) -> bool {
		let imin = bounds.min().to_ivec();
		let imax = bounds.max().to_ivec();

		for iz in imin.z..(imax.z + 1) {
			for iy in imin.y..(imax.y + 1) {
				for ix in imin.x..(imax.x + 1) {
					let pos = ivec3(ix, iy, iz);
					if self.at(pos) != Voxel::EMPTY {
						return true;
					}
				}
			}
		}
		false
	}

	fn update_vao(&self, chnk: uvec2) {
		//println!("Chunks::update_vao {}", chnk);
		let world_offset = ivec3(chnk.x as i32, 0, chnk.y as i32) * (Chunk::XZSIZE as i32);

		let mut b: Vec<MeshBuffer> = zeros(Voxel::MAX);

		let neighbors = [
			ivec3(-1, 0, 0), // left
			ivec3(1, 0, 0),  // right
			ivec3(0, -1, 0), // bottom
			ivec3(0, 1, 0),  // top
			ivec3(0, 0, -1), // back
			ivec3(0, 0, 1),  // front
		];

		let size = self.chunk(chnk).size();
		for iz in 0..size.z {
			for iy in 0..size.y {
				for ix in 0..size.x {
					// voxel coordinates inside chunk
					let vox_int = uvec3(ix, iy, iz);
					let block = self.chunk(chnk).at_internal(vox_int);

					// global voxel coordinates
					let vox_world = world_offset + uvec3(ix, iy, iz).as_ivec();
					let cube_corner = vox_world.map(|v| v as f32);

					if block != Voxel::EMPTY {
						let cube_faces = cube_at(cube_corner);

						for (i, &d) in neighbors.iter().enumerate() {
							let neigh = vox_world + d;
							if self.at(neigh) == Voxel::EMPTY {
								let mut face = self.with_light(i, &cube_faces[i]);
								if block.is_emissive() {
									face = Self::with_emission(&face, block.emission())
								}
								b[block.tex_id()].push_all(&(face.triangle_vertices()))
							}
						}
					}
				}
			}
		}

		let mut meshes = Vec::new();
		for (i, builder) in b.iter().enumerate() {
			if builder.len() != 0 {
				let m = builder.build();
				meshes.push((i, m));
			}
		}
		*(self.chunk(chnk).vaos.borrow_mut()) = Some(meshes);
	}

	fn with_emission(face: &Quad, emission: vec3) -> Quad {
		let mut result = face.clone();
		for i in 0..4 {
			result.set_vertex(
				i,
				Vertex {
					attrib: 0.5 * face.vertex(i).attrib + 0.5 * emission,
					..face.vertex(i).clone()
				},
			)
		}
		result
	}

	fn with_light(&self, face_dir: usize, face: &Quad) -> Quad {
		let mut face = face.clone();

		let n = face.normal();
		let t1 = face.tangent1();
		let t2 = face.tangent2();
		for i in 0..4 {
			let attrib = self.calc_vertex_light(face_dir, face.vertex(i).pos, n, t1, t2);
			face.set_vertex(i, Vertex { attrib, ..face.vertex(i).clone() })
		}
		face
	}

	fn calc_vertex_light(&self, face_dir: usize, pos: vec3, n: vec3, t1: vec3, t2: vec3) -> vec3 {
		let probe_starts = [
			pos + 0.5 * (n + t1 + t2), //
			pos + 0.5 * (n - t1 + t2), //
			pos + 0.5 * (n + t1 - t2), //
			pos + 0.5 * (n - t1 - t2), //
		];

		let face_light = [
			0.65, // left
			0.65, // right
			0.15, // bottom
			1.00, // top
			0.45, // back
			0.45, //front
		];

		let mut accum = vec3::ZERO;
		let mul = face_light[face_dir];

		match face_dir {
			3 => {
				// top
				for &start in &probe_starts {
					accum += self.light_v_probe(start, mul)
				}
			}
			2 => {
				//bottom
				for &start in &probe_starts {
					accum += self.light_l_probe(start, mul)
				}
			}
			_ => {
				// vertical
				for &start in &probe_starts[0..2] {
					// top vertices: probe up
					accum += self.light_v_probe(start, mul)
				}
				for &start in &probe_starts[2..4] {
					// bottom vertices: probe local
					accum += self.light_l_probe(start, mul)
				}
			}
		}

		0.25 * accum
	}

	fn light_v_probe(&self, start: vec3, mul: f32) -> vec3 {
		let start = start.map(|v| v as i32);

		const SHADOW_H: i32 = 10;
		let mut ceil_dist = 0;
		for i in 0..SHADOW_H {
			let up = start + ivec3(0, i, 0);
			if self.at(up) == Voxel::EMPTY {
				ceil_dist += 1;
			} else {
				break;
			}
		}
		let ambient_light = ((ceil_dist) as f32 / (SHADOW_H as f32)) * vec3::ONES;
		if self.at(start).is_emissive() {
			0.25 * mul * ambient_light + (4.0 * 0.75) * self.at(start).emission()
		} else {
			mul * ambient_light
		}
	}

	fn light_l_probe(&self, start: vec3, mul: f32) -> vec3 {
		let start = start.map(|v| v as i32);

		let ambient_light = if self.at(start) == Voxel::EMPTY { vec3::ONES } else { vec3::ZERO };
		if self.at(start).is_emissive() {
			0.25 * mul * ambient_light + (4.0 * 0.75) * self.at(start).emission()
		} else {
			mul * ambient_light
		}
	}

	fn invalidate_vao(&mut self, chnk: uvec2) {
		//println!("Chunks::invalidate_vao {}", chnk);
		*(self.chunk_mut(chnk).vaos.borrow_mut()) = None;
	}

	fn ensure_vao(&self, chnk: uvec2) {
		if self.chunk(chnk).vaos.borrow().is_none() {
			self.update_vao(chnk)
		}
	}

	pub fn ensure_vaos(&mut self) {
		for iz in 0..self.chunks_dim.y {
			for ix in 0..self.chunks_dim.x {
				self.ensure_vao(uvec2(ix, iz));
			}
		}
	}

	#[inline]
	fn index_internal(&self, index: ivec3) -> (uvec2, uvec3) {
		debug_assert!(self.valid_index(index));
		let chnkx = (index.x as u32) >> Chunk::POW;
		let chnkz = (index.z as u32) >> Chunk::POW;
		let intx = (index.x as u32) & Chunk::MASK;
		let intz = (index.z as u32) & Chunk::MASK;
		(uvec2(chnkx, chnkz), uvec3(intx, index.y as u32, intz))
	}

	fn chunk(&self, xz: uvec2) -> &Chunk {
		let idx = self.chunk_index(xz);
		&self.chunks[idx]
	}

	fn chunk_mut(&mut self, xz: uvec2) -> &mut Chunk {
		let idx = self.chunk_index(xz);
		&mut self.chunks[idx as usize]
	}

	fn chunk_index(&self, xz: uvec2) -> usize {
		debug_assert!(self.valid_chunk_index(xz.as_ivec()));
		let stride = self.world_size.x >> Chunk::POW;
		let idx = xz.y * stride + xz.x;
		idx as usize
	}

	fn valid_index(&self, index: ivec3) -> bool {
		!(index.x < 0 || index.y < 0 || index.z < 0 || index.x >= self.world_size.x as i32 || index.y >= self.world_size.y as i32 || index.z >= self.world_size.z as i32)
	}

	fn valid_chunk_index(&self, xz: ivec2) -> bool {
		xz.x >= 0 && xz.y >= 0 && xz.x < self.chunks_dim.x as i32 && xz.y < self.chunks_dim.y as i32
	}

	pub fn world_size(&self) -> uvec3 {
		self.world_size
	}

	// How high is the map at position (ix, iz)?
	// I.e.: y-index+1 of the highest non-empty voxel.
	fn height_at(&self, ix: usize, iz: usize) -> usize {
		let max = self.world_size().y as usize;
		let mut height = 0;

		for iy in 0..max {
			if self.at(ivec3(ix as i32, iy as i32, iz as i32)) != Voxel::EMPTY {
				height = iy;
			}
		}

		height + 1
	}

	pub fn draw(&self, ctx: &GLContext) {
		ctx.textures().bind_voxels();
		ctx.set_depth_test(true);
		ctx.set_cull_face(true);
		let shader = ctx.shaders().bind_voxel_shader();

		for iz in 0..self.chunks_dim.y {
			for ix in 0..self.chunks_dim.x {
				let chnk = uvec2(ix, iz);

				if self.chunk(chnk).vaos.borrow().is_none() {
					self.ensure_vao(chnk)
				}

				let chunk = self.chunk(chnk);
				let meshes = chunk.vaos.borrow();
				let meshes = meshes.as_ref();
				if let Some(meshes) = meshes {
					for (texid, mesh) in meshes {
						shader.set_texture(*texid);
						mesh.bind_and_draw();
					}
				}
			}
		}
	}
}

impl Chunk {
	const POW: u32 = 3; // Chunks will be 2^POW by 2^POW voxels in X,Z.
	const XZSIZE: u32 = 1 << Chunk::POW; // 0b10000
	const MASK: u32 = (1 << Chunk::POW) - 1; // 0b01111

	pub fn new() -> Self {
		Self {
			voxels: Vec::new(),
			vaos: RefCell::new(None),
		}
	}

	pub fn at_internal(&self, idx: uvec3) -> Voxel {
		//println!("Chunk::at_internal {}", idx);
		let (ix, iy, iz) = (idx.x, idx.y, idx.z);
		let i = Self::index_internal(ix, iy, iz);
		if i >= self.voxels.len() {
			Voxel::EMPTY
		} else {
			self.voxels[i]
		}
	}

	pub fn set_internal(&mut self, idx: uvec3, v: Voxel) {
		let (ix, iy, iz) = idx.into();
		let i = Self::index_internal(ix, iy, iz);
		while i >= self.voxels.len() {
			self.voxels.push(Voxel::EMPTY)
		}
		self.voxels[i] = v
	}

	fn index_internal(ix: u32, iy: u32, iz: u32) -> usize {
		debug_assert!(ix < Self::XZSIZE);
		debug_assert!(iz < Self::XZSIZE);
		let idx = (iy << (2 * Self::POW) | iz << Self::POW | ix) as usize;
		//println!("Chunk::index_internal {}, {}, {} -> {}", ix, iy, iz, idx);
		idx
	}

	pub fn size(&self) -> uvec3 {
		uvec3(Self::XZSIZE, self.height(), Self::XZSIZE)
	}

	pub fn height(&self) -> u32 {
		((self.voxels.len() >> (2 * Self::POW)) + 1) as u32
	}
}

impl Default for Chunk {
	fn default() -> Self {
		Self::new()
	}
}

impl Serialize for VoxelBox {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
		VoxelData::from(&self).serialize(serializer)
	}
}

impl<'de> Deserialize<'de> for VoxelBox {
	fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		Ok(VoxelData::deserialize(deserializer)?.into())
	}
}

// Helper data struct to serialize/deserialize a Voxelbox
// without having to use low-level serde functionality.
// Also, encodes a sparse wold relatively efficiently.
#[derive(Serialize, Deserialize)]
struct VoxelData {
	world_size: uvec3,
	voxel_data: Vec<Vec<Vec<u8>>>,
}

impl VoxelData {
	fn from(map: &VoxelBox) -> Self {
		let world_size = map.world_size();
		let mut voxel_data = Vec::new();

		for iz in 0..(world_size.z as usize) {
			voxel_data.push(Vec::new());
			for ix in 0..(world_size.x as usize) {
				voxel_data[iz].push(Vec::new());
				let height = map.height_at(ix, iz);
				for iy in 0..height {
					voxel_data[iz][ix].push(map.at(ivec3(ix as i32, iy as i32, iz as i32)).0)
				}
			}
		}

		Self { world_size, voxel_data }
	}

	fn into(self) -> VoxelBox {
		let map_data = &self.voxel_data;
		let mut map = VoxelBox::new(self.world_size);
		for iz in 0..map_data.len() {
			for ix in 0..map_data[iz].len() {
				for iy in 0..map_data[iz][ix].len() {
					map.set(ivec3(ix as i32, iy as i32, iz as i32), Voxel(map_data[iz][ix][iy]))
				}
			}
		}
		map
	}
}

mod test {
	pub use super::*;

	#[test]
	fn chunk() {
		let mut c = Chunk::new();
		assert_eq!(c.at_internal(uvec3(0, 0, 0)), Voxel::EMPTY);
		assert_eq!(c.at_internal(uvec3(0, 1, 0)), Voxel::EMPTY);
		assert_eq!(c.at_internal(uvec3(5, 99, 7)), Voxel::EMPTY);
		c.set_internal(uvec3(5, 99, 7), Voxel::PLASMA);
		assert_eq!(c.at_internal(uvec3(5, 99, 7)), Voxel::PLASMA);
	}

	#[test]
	fn chunks() {
		let mut c = VoxelBox::new(uvec3(512, 128, 256));
		assert_eq!(c.at(ivec3(0, 0, 0)), Voxel::EMPTY);
		assert_eq!(c.at(ivec3(511, 0, 0)), Voxel::EMPTY);
		assert_eq!(c.at(ivec3(0, 127, 0)), Voxel::EMPTY);
		assert_eq!(c.at(ivec3(0, 0, 255)), Voxel::EMPTY);

		assert_eq!(c.at(ivec3(512, 0, 0)), Voxel::EMPTY);
		assert_eq!(c.at(ivec3(0, 128, 0)), Voxel::EMPTY);
		assert_eq!(c.at(ivec3(0, 0, 256)), Voxel::EMPTY);
		assert_eq!(c.at(ivec3(999, 999, 999)), Voxel::EMPTY);

		for &pos in &[
			ivec3(0, 0, 0),   //
			ivec3(1, 0, 0),   //
			ivec3(0, 1, 0),   //
			ivec3(0, 0, 1),   //
			ivec3(15, 0, 0),  //
			ivec3(0, 15, 0),  //
			ivec3(0, 0, 15),  //
			ivec3(16, 0, 0),  //
			ivec3(0, 16, 0),  //
			ivec3(0, 0, 16),  //
			ivec3(17, 0, 0),  //
			ivec3(0, 17, 0),  //
			ivec3(0, 0, 17),  //
			ivec3(511, 0, 0), //
			ivec3(0, 127, 0), //
			ivec3(0, 0, 255), //
		] {
			c.set(pos, Voxel::PLASMA);
			let (got, want) = (c.at(pos), Voxel::PLASMA);
			if got != want {
				panic!("at {}: got: {:?}, want: {:?}", pos, got, want);
			}
			c.set(pos, Voxel::EMPTY);
		}
	}
}
