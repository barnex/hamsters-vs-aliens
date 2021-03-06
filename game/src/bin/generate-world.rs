use game::prelude::*;
use game::procedural_worlds::*;
use rand::Rng;

fn main() -> Result<()> {
	the_mountain().save("the_mountain.json.gz")?;
	king_off_the_hill().save("king_of_the_hill.json.gz")?;
	Ok(())
}

fn king_off_the_hill() -> Map {
	let size = uvec3(512, 256, 512);
	let mut map = Map::new(size);
	map.background_color = vec3(1.0, 1.0, 1.0);
	map.fog_dist = 150.0;
	let m = &mut map;

	let size = size.map(|v| v as f32);
	let mid = vec3(size.x / 2.0, 0.0, size.z / 2.0);

	// top platform
	//slab(m, mid + (y - 1.0) * vec3::EY, vec3(20.0, 4.0, 20.0), Voxel::PLASMA);

	// spiral stairs
	{
		let n = 150;
		for t in 0..n {
			let t = (t as f32) / (n as f32);

			let th0 = PI / 4.0;
			let th = th0 + t * 3.0 * PI;

			let r0 = 4.0;
			let r1 = 64.0;
			let r = r0 + t * (r1 - r0);

			let y0 = 75.0;
			let y1 = 0.0;
			let y = y0 + t * (y1 - y0);

			let x = r * f32::cos(th);
			let z = r * f32::sin(th);

			let sz0 = 8.0;
			let sz1 = 15.0;
			let sz = sz0 + (sz1 - sz0) * t;

			let pos = mid + vec3(x, y, z);
			disc(m, pos, sz, Voxel::SPONGE);
			disc(m, pos + vec3::EY, sz - 1.0, Voxel::EMPTY);
			disc(m, pos + 2.0 * vec3::EY, sz - 1.0, Voxel::EMPTY);

			let pos = mid + vec3(-x, y, -z);
			disc(m, pos, sz, Voxel::WHITERSTONE);
			disc(m, pos + vec3::EY, sz - 1.0, Voxel::EMPTY);
			disc(m, pos + 2.0 * vec3::EY, sz - 1.0, Voxel::EMPTY);
		}
	}

	//hill
	heightmap(m, mid, ivec2(100, 100), Voxel::GREYSTONE, Voxel::SAND, |x, z| {
		let r = (x * x + z * z + 0.01).sqrt() / 9.0;
		let y = 70.0 * (r.sin() / r).powf(0.5) + 8.0 * ((x / 14.0).sin() + (z / 6.0).cos() + (x / 7.0 + z / 5.0).sin() + 1.5 * (x / 9.0 + z / 7.0).cos() * 0.9 * (x / 11.0 + z / 13.0).sin());
		let y = y * f32::exp(-r * r / (2.0 * PI));
		let mut y = y;
		if y > 75.0 {
			y = 75.0
		}
		y
	});

	slab(m, mid + vec3(0.0, 75.0, 0.0), vec3(80.0, 8.0, 80.0), Voxel::EMPTY);

	for x in -16..16 {
		for z in -16..16 {
			let x = (x * 16) as f32;
			let z = (z * 16) as f32;

			let y = 2.0;

			let pos = vec3(x, y, z) + 4.0 * rand_vec();
			slab(m, mid + pos, vec3(10.0, 4.0, 10.0), Voxel::WHITESTONE);
		}
	}

	floor(m, 2, Voxel::SNOW);

	map
}

fn rand_vec() -> vec3 {
	vec3(
		rand::thread_rng().gen_range(-0.5..0.5),
		rand::thread_rng().gen_range(-0.5..0.5),
		rand::thread_rng().gen_range(-0.5..0.5),
	)
}

fn slab(m: &mut Map, pos: vec3, size: vec3, v: Voxel) {
	range(m, pos - size / 2.0, pos + size / 2.0, v)
}

fn range(m: &mut Map, min: vec3, max: vec3, v: Voxel) {
	irange(m, min.map(|v| v as i32), max.map(|v| v as i32), v)
}

fn irange(m: &mut Map, min: ivec3, max: ivec3, v: Voxel) {
	for iz in min.z..max.z {
		for ix in min.x..max.x {
			for iy in min.y..max.y {
				m.set(ivec3(ix, iy, iz), v)
			}
		}
	}
}

fn the_mountain() -> Map {
	let mut m = Map::new(uvec3(256, 64, 256));
	let (nx, _, nz) = m.size().as_ivec().into();
	let mid = ivec3(nx / 2, 0, nz / 2);
	let mid = mid.map(|v| v as f32);

	// dunes
	heightmap(&mut m, mid, ivec2(nx, nz), Voxel::SNOW, Voxel::SNOW, |x, z| {
		let x = x / 90.0;
		let z = z / 90.0;
		1.8 * ((7.0 * x + 9.0 * z).sin() + (5.0 * x + 3.0 * z).cos() + (3.0 * x + 8.0 * z).cos()) + 5.0
	});

	// rocks
	heightmap(&mut m, mid, ivec2(nx, nz), Voxel::WHITESTONE, Voxel::WHITERSTONE, |x, z| {
		let x = x / 90.0;
		let z = z / 90.0;
		9.8 * ((17.0 * x + 19.0 * z).cos() + (15.0 * x + 13.0 * z).sin() + (13.0 * x + 11.0 * z).sin()) - 20.0
	});

	// lava basin
	floor(&mut m, 5, Voxel::LAVA);

	// mountain
	heightmap(&mut m, mid, ivec2(32, 32), Voxel::GREYSTONE, Voxel::SNOW, |x, z| {
		let r = (x * x + z * z + 0.01).sqrt() / 3.5;
		40.0 * (r.sin() / r).powf(0.8) + 8.0 * ((x / 14.0).sin() + (z / 6.0).cos() + (x / 7.0 + z / 5.0).sin() + 1.5 * (x / 9.0 + z / 7.0).cos() * 0.9 * (x / 11.0 + z / 13.0).sin()) + 1.0
	});

	// islands
	heightmap(&mut m, mid, ivec2(nx, nz), Voxel::WHITESTONE, Voxel::WHITESTONE, |x, z| {
		let x = x / 19.0;
		let z = z / 19.0;
		2.5 * ((5.0 * x + 9.0 * z).sin() + (5.0 * x + 3.0 * z).cos() + (3.0 * x + 8.0 * z).cos()).powf(0.5) + 1.9
	});

	// tunnel
	let mid = mid.map(|v| v as f32);
	slab(&mut m, mid + vec3(0.0, 9.0, 0.0), vec3(40.0, 4.0, 2.0), Voxel::EMPTY);
	slab(&mut m, mid + vec3(0.0, 9.0, 0.0), vec3(2.0, 4.0, 40.0), Voxel::EMPTY);
	m
}

// fn to_f32<T: Into<f32>>(v: T) -> f32 {
// 	v.into()
// }
