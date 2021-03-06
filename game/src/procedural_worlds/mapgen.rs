use crate::prelude::*;

pub fn heightmap<F: Fn(f32, f32) -> f32>(m: &mut Map, pos: vec3, size: ivec2, fill: Voxel, top: Voxel, f: F) {
	let pos = pos.map(|v| v as i32);
	let (nx, _, nz) = m.size().as_ivec().into();
	for iz in (pos.z - size.y / 2)..(pos.z + size.y / 2) {
		for ix in (pos.x - size.x / 2)..(pos.x + size.x / 2) {
			let x = (ix - nx / 2) as f32;
			let z = (iz - nz / 2) as f32;
			let h = f(x, z);
			let iy = h as i32 + pos.y;
			for y in 0..iy {
				m.set(ivec3(ix, y, iz), fill);
			}
			m.set(ivec3(ix, iy, iz), top);
		}
	}
}

pub fn floor(m: &mut Map, height: i32, v: Voxel) {
	let (nx, _, nz) = m.size().as_ivec().into();
	range(m, ivec3(0, 0, 0), ivec3(nx, height, nz), v)
}

pub fn slab(m: &mut Map, pos: ivec3, size: ivec3, v: Voxel) {
	range(m, pos - size / 2, pos + size / 2, v)
}

pub fn disc(m: &mut Map, pos: vec3, r: f32, v: Voxel) {
	let min = (pos - vec3(r, 0.0, r)).map(|v| v as i32);
	let max = (pos + vec3(r, 1.0, r)).map(|v| v as i32);
	for iz in min.z..max.z {
		for ix in min.x..max.x {
			for iy in min.y..max.y {
				let p = vec3(ix as f32, iy as f32, iz as f32);
				if (p - pos).len() < r {
					m.set(ivec3(ix, iy, iz), v)
				}
			}
		}
	}
}

pub fn range(m: &mut Map, min: ivec3, max: ivec3, v: Voxel) {
	for iz in min.z..max.z {
		for ix in min.x..max.x {
			for iy in min.y..max.y {
				m.set(ivec3(ix, iy, iz), v)
			}
		}
	}
}
