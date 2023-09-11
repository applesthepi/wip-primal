use crate::TilePositionAbs;

#[derive(Default, Clone, Copy)]
pub struct Bounds {
	min: TilePositionAbs,
	max: TilePositionAbs,
}

impl Bounds {
	pub fn new(
		min: TilePositionAbs,
		max: TilePositionAbs,
	) -> Self {
		Self {
			min,
			max,
		}
	}

	pub fn from_origin(
		origin: TilePositionAbs,
		half_size: TilePositionAbs,
	) -> Self {
		Self {
			min: origin - half_size,
			max: origin + half_size,
		}
	}
}