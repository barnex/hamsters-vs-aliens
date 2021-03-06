use super::internal::*;

pub struct TexturePack {
	voxels: Vec<Texture>,
	skins: Vec<Texture>,
}

impl TexturePack {
	pub fn new(texture_dir: &Path) -> Self {
		let voxels = Self::load(
			texture_dir,
			&[
				("snow", WHITE),             //
				("lava", RED),               //
				("plasma", GREEN),           //
				("whitestone2", LIGHT_GREY), //
				("greystone", DARK_GREY),    //
				("sand", YELLOW),            //
				("whiterstone", GREY),       //
				("stars", MAGENTA),          //
				("sponge", ORANGE),          //
				("crystal", BLUE),           //
			],
		);
		let skins = Self::load(
			texture_dir,
			&[
				("alien", GREEN),   //
				("hamster", BROWN), //
				("frog", GREEN),    //
				("chicken", WHITE), //
				("xy", WHITE),      //
			],
		);
		Self { voxels, skins }
	}

	/// Load the listed texture files (names without extension) from a directory.
	/// On error, very simple fallback textures are returned.
	pub fn load(dir: &Path, files: &[(&str, vec3)]) -> Vec<Texture> {
		files.iter().map(|f| Self::load_mipmapped(dir, f.0, f.1)).collect()
	}

	/// Bind this pack's voxel textures to consecutive texture units, starting from 0.
	pub fn bind_voxels(&self) {
		Self::bind_all(&self.voxels)
	}

	/// Bind this pack's skin textures to consecutive texture units, starting from 0.
	pub fn bind_skins(&self) {
		Self::bind_all(&self.skins)
	}

	pub fn bind_all(textures: &[Texture]) {
		for (i, tex) in textures.iter().enumerate() {
			tex.bind_texture_unit(i as u32);
		}
	}

	fn load_mipmapped(dir: &Path, base: &str, replacement_color: vec3) -> Texture {
		let fname = dir.join(base).with_extension("jpg");
		println!("loading {}", fname.to_string_lossy());

		match load_texture(&fname) {
			Err(e) => {
				eprintln!("load {}: {}", fname.to_string_lossy(), e);
				Self::fallback_texture(replacement_color)
			}
			Ok(tex) => tex
				.wrap_repeat()
				.generate_mipmap()
				.parameteri(gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32)
				.parameteri(gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32),
		}
	}

	// simple 2x2 texture to be used when loading the real texture failed.
	fn fallback_texture(replacement_color: vec3) -> Texture {
		let size = uvec2(2, 2);
		//let data: [[u8; 4]; 4] = [
		//	[255, 255, 255, 255], //
		//	[220, 220, 255, 255], //
		//	[255, 220, 220, 255], //
		//	[220, 255, 220, 255], //
		//];
		let r = (replacement_color.x * 255.0) as u8;
		let g = (replacement_color.y * 255.0) as u8;
		let b = (replacement_color.z * 255.0) as u8;
		let data: [[u8; 4]; 4] = [
			[r, g, b, 255], //
			[r, g, b, 255], //
			[r, g, b, 255], //
			[r, g, b, 255], //
		];

		let levels = 1;
		Texture::new2d(gl::RGBA8, levels, size) //
			.sub_image2d(0, 0, 0, size.x, size.y, gl::RGBA, gl::UNSIGNED_BYTE, &data)
			.wrap_repeat()
			.parameteri(gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32)
			.parameteri(gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32)
	}
}
