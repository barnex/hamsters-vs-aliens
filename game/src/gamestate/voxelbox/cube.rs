use super::internal::*;

// pub enum FaceDir {
// 	Left = 0,
// 	Right = 1,
// 	Bottom = 2,
// 	Top = 3,
// 	Back = 4,
// 	Front = 5,
// }

pub fn cube_at(corner_pos: vec3) -> [Quad; 6] {
	let tex_stretch = 1.0 / 8.0;

	let vx = |x: i32, (tx, ty): (i32, i32), norm: vec3| -> Vertex {
		let pos = ivec3(x, tx, ty).to_vec() + corner_pos;
		let tex = tex_stretch * vec2(pos.y + pos.x, pos.z + pos.x);
		Vertex::new(pos, tex, norm)
	};

	let vy = |y: i32, (tx, ty): (i32, i32), norm: vec3| -> Vertex {
		let pos = ivec3(tx, y, ty).to_vec() + corner_pos;
		let tex = tex_stretch * vec2(pos.x + pos.y, pos.z + pos.y);
		Vertex::new(pos, tex, norm)
	};

	let vz = |z: i32, (tx, ty): (i32, i32), norm: vec3| -> Vertex {
		let pos = ivec3(tx, ty, z).to_vec() + corner_pos;
		let tex = tex_stretch * vec2(pos.x + pos.z, pos.y + pos.z);
		Vertex::new(pos, tex, norm)
	};

	const EX: vec3 = vec3::EX;
	const EY: vec3 = vec3::EY;
	const EZ: vec3 = vec3::EZ;

	let front = Quad::new(
		vz(1, (0, 0), EZ), //
		vz(1, (0, 1), EZ), //
		vz(1, (1, 1), EZ), //
		vz(1, (1, 0), EZ), //
	);

	let back = Quad::new(
		vz(0, (0, 0), -EZ), //
		vz(0, (1, 0), -EZ), //
		vz(0, (1, 1), -EZ), //
		vz(0, (0, 1), -EZ), //
	);

	let left = Quad::new(
		vx(0, (0, 0), -EX), //
		vx(0, (1, 0), -EX), //
		vx(0, (1, 1), -EX), //
		vx(0, (0, 1), -EX), //
	);

	let right = Quad::new(
		vx(1, (0, 0), EX), //
		vx(1, (0, 1), EX), //
		vx(1, (1, 1), EX), //
		vx(1, (1, 0), EX), //
	);

	let bottom = Quad::new(
		vy(0, (0, 0), -EY), //
		vy(0, (0, 1), -EY), //
		vy(0, (1, 1), -EY), //
		vy(0, (1, 0), -EY), //
	);

	let top = Quad::new(
		vy(1, (0, 0), EY), //
		vy(1, (1, 0), EY), //
		vy(1, (1, 1), EY), //
		vy(1, (0, 1), EY), //
	);

	[left, right, bottom, top, back, front]
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn cube_faces() {
		let c = cube_at(vec3(1.0, 2.0, 3.0));

		assert_eq!(c[0].normal(), vec3(-1.0, 0.0, 0.0));
		assert_eq!(c[1].normal(), vec3(1.0, 0.0, 0.0));
		assert_eq!(c[2].normal(), vec3(0.0, -1.0, 0.0));
		assert_eq!(c[3].normal(), vec3(0.0, 1.0, 0.0));
		assert_eq!(c[4].normal(), vec3(0.0, 0.0, -1.0));
		assert_eq!(c[5].normal(), vec3(0.0, 0.0, 1.0));

		assert_eq!(c[5].tangent1(), vec3(0.0, 1.0, 0.0));
		assert_eq!(c[5].tangent2(), vec3(1.0, 0.0, 0.0));
	}
}
