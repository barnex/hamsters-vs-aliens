use super::internal::*;

pub fn load_texture(fname: &Path) -> Result<Texture> {
	load_image_nomsg(fname) //.msg(&format!("load {}", fname))
}

fn load_image_nomsg(fname: &Path) -> Result<Texture> {
	let src = image::io::Reader::open(fname)?.decode()?.into_rgba8();
	let size = uvec2(src.width(), src.height());
	let mut data = Vec::with_capacity((size.x as usize) * (size.y as usize));
	for c in src.pixels() {
		data.push([c[0], c[1], c[2], c[3]])
	}
	let levels = 6;
	Ok(Texture::new2d(gl::RGBA8, levels, size) //
		.sub_image2d(0, 0, 0, size.x, size.y, gl::RGBA, gl::UNSIGNED_BYTE, &data))
}
