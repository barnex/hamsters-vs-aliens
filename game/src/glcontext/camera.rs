use super::internal::*;

const Z_FAR: f32 = 400.0;
const Z_NEAR: f32 = 0.5; // also focal length. 0.5 = 90 deg.

pub fn camera_matrix(viewport: (u32, u32), pos: vec3, yaw: f32, pitch: f32) -> mat4 {
	// Scale geometry and far plane by s.
	// Equivalent to brining near plane closer to the lens while keeping the FOV.
	// Value chosen so that objects of absolute size ~1.0 can be viewed without any discernable near clipping.
	let s = 10.0;
	let (w, h) = (viewport.0 as f32, viewport.1 as f32);
	let (w, h) = (1.0, h / w);
	let (w2, h2) = (w / 2.0, h / 2.0);
	let (z1, z2) = (Z_NEAR, Z_FAR * s);

	//let view = &yaw_pitch_matrix(self.yaw as f32, self.pitch as f32);
	//let view = &mat4::UNIT;
	// http://docs.gl/gl3/glFrustum

	let proj = &mat4::transpose([
		[z1 / w2, 0.0, 0.0, 0.0],
		[0.0, z1 / h2, 0.0, 0.0],
		[0.0, 0.0, -(z1 + z2) / (z2 - z1), -2.0 * z1 * z2 / (z2 - z1)],
		[0.0, 0.0, -1.0, 0.0],
	]) * &mat4::transpose([
		[s, 0.0, 0.0, 0.0],   //
		[0.0, s, 0.0, 0.0],   //
		[0.0, 0.0, s, 0.0],   //
		[0.0, 0.0, 0.0, 1.0], //
	]);

	let rotate = yaw_pitch_matrix(yaw, pitch);
	let translate = translation_matrix(-pos);
	let matrix = &proj * &rotate * &translate; // * &swap_yz;
	matrix
}

pub fn isometric_matrix(viewport: (u32, u32)) -> mat4 {
	// Scale geometry and far plane by s.
	// Equivalent to brining near plane closer to the lens while keeping the FOV.
	// Value chosen so that objects of absolute size ~1.0 can be viewed without any discernable near clipping.
	let (w, h) = (viewport.0 as f32, viewport.1 as f32);
	let (w, h) = (1.0, h / w);
	let (w2, h2) = (w / 2.0, h / 2.0);

	mat4::transpose([
		[1.0 / w2, 0.0, 0.0, 0.0], //
		[0.0, 1.0 / h2, 0.0, 0.0],
		[0.0, 0.0, 1.0, 0.0],
		[0.0, 0.0, 0.0, 1.0],
	])
}
