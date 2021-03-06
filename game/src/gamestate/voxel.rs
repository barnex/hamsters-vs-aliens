use super::internal::*;

/// All the voxel (cube) types: sand, lava, snow,...
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Voxel(pub u8);

impl Voxel {
	pub const EMPTY: Voxel = Voxel(0);
	pub const SNOW: Voxel = Voxel(1);
	pub const LAVA: Voxel = Voxel(2);
	pub const PLASMA: Voxel = Voxel(3);
	pub const WHITESTONE: Voxel = Voxel(4);
	pub const GREYSTONE: Voxel = Voxel(5);
	pub const SAND: Voxel = Voxel(6);
	pub const WHITERSTONE: Voxel = Voxel(7);
	pub const STARS: Voxel = Voxel(8);
	pub const SPONGE: Voxel = Voxel(9);
	pub const CRYSTAL: Voxel = Voxel(10);
	pub const MAX: usize = 10;

	/// The index of this Voxel's texture, in VoxelBox.texture_pack.
	pub fn tex_id(self) -> usize {
		debug_assert!(self != Voxel::EMPTY);
		(self.0 as usize) - 1
	}

	/// Color of the light emitted by this voxel, if any.
	pub fn emission(self) -> vec3 {
		match self {
			Voxel::LAVA => vec3(1.0, 0.3, 0.0),
			Voxel::CRYSTAL => vec3(0.1, 0.4, 1.0),
			_ => vec3(0.0, 0.0, 0.0),
		}
	}

	/// Does this voxel emit light?
	pub fn is_emissive(self) -> bool {
		match self {
			Voxel::LAVA | Voxel::CRYSTAL => true,
			_ => false,
		}
	}
}
