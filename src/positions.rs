/// # ABS/REL/OFF
/// 
/// ABS - Absolute world position
/// REL - Relative to higher abstraction
/// 	(TilePositionRel is releative to its owner ChunkPositionAbs)
/// 	(SectorPositionRel doesn't exist because it can't be relative to anything)
/// OFF - Offset from the same abstraction
/// 
/// # Function Referance
/// 
/// ABS -> ABS | `TilePositionAbs::into_chunk() -> ChunkPositionAbs;` / `TilePositionAbs::as_chunk() -> ChunkPositionAbs;`
/// ABS -> ABS | `TilePositionAbs::from_chunk(&ChunkPositionAbs, {integer}, {integer}) -> TilePositionAbs;`
/// ABS -> REL | `ChunkPositionAbs::rel_from_abs(&TilePositionAbs) -> ChunkPositionRel;`
/// REL -> ABS | `TilePositionRel::into_abs(&ChunkPositionAbs) -> TilePositionAbs`

use wip_pm::WorldPosition;

use crate::{CHUNK_WIDTH, SECTOR_WIDTH};

// TODO: redo PM to generate these sub methods.
// TODO: not all methods are impl for each position type.
// TODO: create OFF types for each position type.
// TODO: incorperate OFF types into referance.

#[derive(WorldPosition, Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct EntityPositionAbs {
	pub x: f64,
	pub y: f64,
}

impl EntityPositionAbs {
	pub fn new(
		x: f64,
		y: f64,
	) -> Self {
		Self { x, y }
	}

	pub fn splat(
		v: f64,
	) -> Self {
		Self { x: v, y: v, }
	}
}

#[derive(WorldPosition, Debug, Default, Clone, Copy, Ord, Eq, PartialEq, PartialOrd)]
pub struct TilePositionAbs {
	pub x: i64,
	pub y: i64,
}

impl TilePositionAbs {
	pub fn new(
		x: i64,
		y: i64,
	) -> Self {
		Self { x, y }
	}

	pub fn splat(
		v: i64,
	) -> Self {
		Self { x: v, y: v, }
	}

	pub fn from_chunk_abs(
		chunk_position_abs: &ChunkPositionAbs,
		tile_position_rel: &TilePositionRel,
	) -> Self {
		Self {
			x: chunk_position_abs.x * CHUNK_WIDTH as i64 + tile_position_rel.x as i64,
			y: chunk_position_abs.y * CHUNK_WIDTH as i64 + tile_position_rel.y as i64,
		}
	}

	pub fn from_chunk_abs_lossy(
		chunk_position_abs: &ChunkPositionAbs,
	) -> Self {
		Self {
			x: chunk_position_abs.x * CHUNK_WIDTH as i64,
			y: chunk_position_abs.y * CHUNK_WIDTH as i64,
		}
	}

	pub fn into_chunk(
		self,
	) -> ChunkPositionAbs {
		ChunkPositionAbs::new(
			(self.x as f64 / CHUNK_WIDTH as f64).floor() as i64,
			(self.y as f64 / CHUNK_WIDTH as f64).floor() as i64,
		)
	}
}

#[derive(WorldPosition, Debug, Default, Clone, Copy, Ord, Eq, PartialEq, PartialOrd)]
pub struct TilePositionRel {
	pub x: u8,
	pub y: u8,
}

impl TilePositionRel {
	pub fn new(
		x: u8,
		y: u8,
	) -> Self {
		Self { x, y }
	}

	pub fn splat(
		v: u8,
	) -> Self {
		Self { x: v, y: v, }
	}

	/// Takes a tile relative to a src chunk and converts
	/// 	it to an absolute tile position in the world.
	/// 
	/// # Arguments
	/// 
	/// * `chunk_position_abs` - chunk absolute position
	/// 	that this tile is relative to.
	pub fn into_abs(
		self,
		chunk_position_abs: ChunkPositionAbs,
	) -> TilePositionAbs {
		TilePositionAbs::new(
			chunk_position_abs.x * CHUNK_WIDTH as i64 + self.x as i64,
			chunk_position_abs.y * CHUNK_WIDTH as i64 + self.y as i64,
		)
	}
}

#[derive(WorldPosition, Debug, Default, Clone, Copy, Ord, Eq, PartialEq, PartialOrd)]
pub struct ChunkPositionAbs {
	pub x: i64,
	pub y: i64,
}

impl ChunkPositionAbs {
	pub const NULL: ChunkPositionAbs = ChunkPositionAbs { x: i64::MAX, y: i64::MAX };

	pub fn new(
		x: i64,
		y: i64,
	) -> Self {
		Self { x, y }
	}

	pub fn splat(
		v: i64,
	) -> Self {
		Self { x: v, y: v, }
	}

	pub fn from_sector(
		sector_position_abs: &SectorPositionAbs,
		x: i64,
		y: i64,
	) -> Self {
		Self {
			x: sector_position_abs.x * SECTOR_WIDTH as i64 + x,
			y: sector_position_abs.y * SECTOR_WIDTH as i64 + y,
		}
	}

	pub fn into_sector(
		self,
	) -> SectorPositionAbs {
		SectorPositionAbs::new(
			(self.x as f64 / SECTOR_WIDTH as f64).floor() as i64,
			(self.y as f64 / SECTOR_WIDTH as f64).floor() as i64,
		)
	}

	pub fn as_sector(
		&self,
	) -> SectorPositionAbs {
		SectorPositionAbs::new(
			(self.x as f64 / SECTOR_WIDTH as f64).floor() as i64,
			(self.y as f64 / SECTOR_WIDTH as f64).floor() as i64,
		)
	}
}

#[derive(WorldPosition, Debug, Default, Clone, Copy, Ord, Eq, PartialEq, PartialOrd)]
pub struct ChunkPositionRel {
	pub x: u16,
	pub y: u16,
}

impl ChunkPositionRel {
	pub fn new(
		x: u16,
		y: u16,
	) -> Self {
		Self { x, y }
	}

	pub fn splat(
		v: u16,
	) -> Self {
		Self { x: v, y: v, }
	}

	pub fn into_abs(
		self,
		sector_position_abs: &SectorPositionAbs,
	) -> ChunkPositionAbs {
		ChunkPositionAbs::new(
			(sector_position_abs.x * SECTOR_WIDTH as i64) + self.x as i64,
			(sector_position_abs.y * SECTOR_WIDTH as i64) + self.y as i64,
		)
	}
}

#[derive(WorldPosition, Debug, Default, Clone, Copy, Ord, Eq, PartialEq, PartialOrd)]
pub struct SectorPositionAbs {
	pub x: i64,
	pub y: i64,
}

impl SectorPositionAbs {
	pub fn new(
		x: i64,
		y: i64,
	) -> Self {
		Self { x, y }
	}

	pub fn splat(
		v: i64,
	) -> Self {
		Self { x: v, y: v, }
	}

	pub fn rel_from_abs(
		&self,
		chunk_position_abs: &ChunkPositionAbs,
	) -> ChunkPositionRel {
		let partial_chunk_position_abs = ChunkPositionAbs::from_sector(&self, 0, 0);
		debug_assert!(*chunk_position_abs <= partial_chunk_position_abs); // TODO: PM somehow allow referance cmp?
		let diff = partial_chunk_position_abs - *chunk_position_abs;
		debug_assert!(diff.x < SECTOR_WIDTH as i64 && diff.y < SECTOR_WIDTH as i64);
		ChunkPositionRel::new(
			diff.x as u16,
			diff.y as u16,
		)
	}

	pub fn attempt_rel_from_abs(
		&self,
		chunk_position_abs: &ChunkPositionAbs,
	) -> Option<ChunkPositionRel> {
		let partial_chunk_position_abs = ChunkPositionAbs::from_sector(
			&self,
			0,
			0,
		);
		if *chunk_position_abs > partial_chunk_position_abs { return None; }
		let diff = partial_chunk_position_abs - *chunk_position_abs;
		if diff.x >= SECTOR_WIDTH as i64 || diff.y >= SECTOR_WIDTH as i64 { return None; }
		Some(ChunkPositionRel::new(
			diff.x as u16,
			diff.y as u16,
		))
	}
}

#[derive(WorldPosition, Debug, Default, Clone, Copy, Ord, Eq, PartialEq, PartialOrd)]
pub struct SectorPositionOff {
	pub x: i64,
	pub y: i64,
}

impl SectorPositionOff {
	pub fn new(
		x: i64,
		y: i64,
	) -> Self {
		Self { x, y }
	}

	pub fn splat(
		v: i64,
	) -> Self {
		Self { x: v, y: v, }
	}
}